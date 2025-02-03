#[cfg(feature = "integration")]
mod integration_tests {
    use anyhow::Context;
    use api::{
        model::MODELS,
        plan::{
            step::{
                query::{
                    slack::SlackQuery, slack::SlackQueryInput, slack::SlackQueryWithData,
                    slack::SlackStep, Query,
                },
                return_data::{ReturnData, ReturnDataSelection},
                Step,
            },
            Confidence, PlanResponse, QueryPlannerInput, QueryStepOutput,
        },
        v1::SourcesResponse,
    };
    use models::source::Sources;
    use reqwest::Client;
    use serde_json::json;
    use sqlx::PgPool;
    use std::{future::Future, ops::Range, time::Duration};

    const PLANNER_PROMPT: &str = include_str!("../prompts/v1.1/planner_v1.txt");
    const SLACK_PROMPT: &str = include_str!("../prompts/v1.1/slack_v1.txt");
    const GOOGLE_DOCS_PROMPT: &str = include_str!("../prompts/v1.1/google_docs_v1.txt");

    #[derive(Copy, Clone)]
    enum CannedUser {
        Dev1,
        Dev2,
        Dev3,
    }

    impl CannedUser {
        fn api_key(self) -> &'static str {
            match self {
                CannedUser::Dev1 => "ArWSOTAe6JShzlKbf5KNND6vTi8WpGQe_tSAAZoqGXYBBmPVK4lVsDVSg8hmhBqRFdSXmkdblHot0ae9fDeZxzFlbUxgYQ7qAap6GlkTJAFpsxKyxmzh8QA",
                CannedUser::Dev2 => "Hn9kCs29WotIaTfWJWo-aaSMwNu0dPfV_LmOROY7CoUsgmr0x26eiSSuPoQLPjR38+nH1w3OFv2hGBd+3UTzkrLen-bKVZGtzzT9I34DMxl-bmBzTPzuQqA",
                CannedUser::Dev3 => "7KPkflaTjZ+CsXHhEu4ge-cTt4iOY9Cl_guhKGcF9YO0Lm9AgC3Tr230++UIS14+M0K8iB53LgcIDyoEIqjDGq50V0HDI4f8pTup+dV8JM-dLLo6DYpT40Q",
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    struct TestResults {
        results: Vec<TestResult>,
    }

    impl TestResults {
        fn new() -> Self {
            Self {
                results: Vec::new(),
            }
        }

        fn add(&mut self, result: TestResult) {
            self.results.push(result);
        }

        fn total_points(&self) -> u32 {
            self.results.iter().map(|r| r.total_points).sum()
        }

        fn earned_points(&self) -> u32 {
            self.results.iter().map(|r| r.earned_points).sum()
        }
    }

    impl std::fmt::Display for TestResults {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for result in &self.results {
                write!(f, "{}", result)?;
            }

            if self.results.len() > 1 {
                writeln!(f, "\n=== Final Score ===")?;
                writeln!(
                    f,
                    "Total: {}/{} points ({:.1}%)",
                    self.earned_points(),
                    self.total_points(),
                    (self.earned_points() as f32 / self.total_points() as f32) * 100.0
                )?;
            }
            Ok(())
        }
    }
    #[derive(Debug, Clone, Default)]
    struct TestResult {
        name: String,
        total_points: u32,
        earned_points: u32,
        details: Vec<CheckResult>,
    }

    impl std::fmt::Display for TestResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "\n=== Test: {} ===", self.name)?;
            writeln!(
                f,
                "Score: {}/{} points ({:.1}%)",
                self.earned_points,
                self.total_points,
                (self.earned_points as f32 / self.total_points as f32) * 100.0
            )?;

            writeln!(f, "\nDetailed Results:")?;
            for detail in &self.details {
                writeln!(
                    f,
                    "{} [{}/{}pts] {}",
                    if detail.passed { "✓" } else { "✗" },
                    detail.earned_points,
                    detail.total_points,
                    detail.description
                )?;
                for test in &detail.failed_tests {
                    writeln!(f, "-> ✗ index: {test}")?;
                }
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, Default)]
    struct CheckResult {
        description: String,
        passed: bool,
        total_points: u32,
        earned_points: u32,
        failed_tests: Vec<usize>,
    }

    impl TestResult {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                ..Default::default()
            }
        }

        async fn start_test_sequence<F>(&mut self, description: &str, total_points: u32, test_fn: F)
        where
            F: FnOnce() -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<Vec<bool>>>>>,
        {
            self.total_points += total_points;
            let mut earned_points = 0;
            let mut failed_tests = vec![];
            match test_fn().await {
                Ok(checks) => {
                    for (i, passed) in checks.into_iter().enumerate() {
                        if passed {
                            earned_points += 1;
                        } else {
                            failed_tests.push(i);
                        }
                    }
                    self.earned_points += earned_points;
                    self.details.push(CheckResult {
                        description: description.to_string(),
                        total_points,
                        earned_points,
                        passed: earned_points == total_points,
                        failed_tests,
                    });
                }
                Err(e) => {
                    eprintln!("Running test: {description}: {e:?}");
                    self.details.push(CheckResult {
                        description: description.to_string(),
                        total_points,
                        earned_points: 0,
                        passed: false,
                        failed_tests: (0..total_points as usize).collect(),
                    });
                }
            }
        }
    }

    async fn start_server(pool: PgPool, port: u16) {
        tokio::spawn(async move {
            api::run(pool, port).await;
        });
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    fn query_planner_step_matches(
        query_step: &Query,
        query: &str,
        sources: Sources,
        min_confidence: Confidence,
    ) -> bool {
        query_step.query.as_str().to_lowercase() == query.to_lowercase()
            && query_step.confidence >= min_confidence
            && query_step.targets.eq(&sources)
    }

    fn query_planner_step_matches_range(
        query_step: &Query,
        query_range: Range<usize>,
        sources: Sources,
        min_confidence: Confidence,
    ) -> bool {
        query_step.query.len() >= query_range.start
            && query_step.query.len() <= query_range.end
            && query_step.confidence >= min_confidence
            && query_step.targets.eq(&sources)
    }

    fn return_planner_step_matches(
        return_step: &ReturnData,
        data_selection: Option<ReturnDataSelection>,
        transform_instruction_range: Option<Range<usize>>,
        min_confidence: Confidence,
        user_message_range: Option<Option<Range<usize>>>,
    ) -> bool {
        (match (&return_step.data_selection, data_selection) {
            (Some(r_data_selection), Some(data_selection)) => r_data_selection.eq(&data_selection),
            (None, None) => true,
            _ => false,
        }) && (match (
            &return_step.transform_instruction,
            transform_instruction_range,
        ) {
            (Some(instruction), Some(range)) => {
                instruction.len() >= range.start && instruction.len() <= range.end
            }
            (None, None) => true,
            _ => false,
        }) && return_step.confidence >= min_confidence
            && (match (&return_step.user_message, user_message_range) {
                (Some(msg), Some(Some(range))) => {
                    msg.len() >= range.start && msg.len() <= range.end
                }
                (None, Some(None)) => true,
                (_, None) => true,
                _ => false,
            })
    }

    async fn get_context(query: &str, port: u16, user: CannedUser) -> anyhow::Result<PlanResponse> {
        let client = Client::new();
        let plan_response: PlanResponse = client
            .post(format!("http://127.0.0.1:{port}/v1/context"))
            .header("Authorization", format!("Bearer {}", user.api_key()))
            .header("X-API-Key", "5nSVZr6pn4TJ6av8BbVOl2XXmCGDqz2D_BcqGvc4jNU9U5fPrdvQyRG+k8rYJhF8JErGZDDz7zyEeT0CVhW6g1NdG0QFoHIc2RYkBHq+6ftCIFJbb72a8LA")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query": "{}"}}"#, query))
            .send()
            .await?
            .json()
            .await?;
        Ok(plan_response)
    }

    async fn get_sources(port: u16, user: CannedUser) -> anyhow::Result<SourcesResponse> {
        let client = Client::new();
        let sources_response: SourcesResponse = client
            .get(format!("http://127.0.0.1:{port}/v1/sources"))
            .header("Authorization", format!("Bearer {}", user.api_key()))
            .header("X-API-Key", "5nSVZr6pn4TJ6av8BbVOl2XXmCGDqz2D_BcqGvc4jNU9U5fPrdvQyRG+k8rYJhF8JErGZDDz7zyEeT0CVhW6g1NdG0QFoHIc2RYkBHq+6ftCIFJbb72a8LA")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(sources_response)
    }

    #[sqlx::test(migrations = "../../migrations", fixtures("setup", "github"))]
    async fn test_sources(pool: PgPool) {
        let port = 30001;
        start_server(pool.clone(), port).await;

        let sources = get_sources(port, CannedUser::Dev1).await.unwrap();
        assert_eq!(
            sources.sources,
            serde_json::from_value(json!({
                "GitHub": [
                    "Issues",
                    "PullRequests",
                    "Repositories",
                    "Comments"
                ]
            }))
            .unwrap()
        );
    }

    #[sqlx::test(migrations = "../../migrations", fixtures("setup", "github", "slack"))]
    async fn score_llm(pool: PgPool) {
        let port = 30000;
        start_server(pool.clone(), port).await;

        let mut test_results = TestResults::new();

        // Direct LLM tests
        test_results.add(score_planner_model().await);
        test_results.add(score_planner_assumptions().await);
        test_results.add(score_slack_model().await);
        test_results.add(score_google_docs_model().await);

        // Full integration API tests
        test_results.add(score_github(port).await);
        test_results.add(score_slack(port).await);
        test_results.add(score_ridiculous_requests(port).await);
        test_results.add(score_no_matching_data_sources(port).await);

        // Full crossover integration API tests
        test_results.add(score_github_slack(port).await);

        println!("{}", test_results);
    }

    async fn score_planner_assumptions() -> TestResult {
        let mut test = TestResult::new("planner assumptions");

        test.start_test_sequence("assumptions test - My GitHub Issues", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": [
                        "Issues",
                        "PullRequests",
                        "Repositories",
                        "Comments"
                    ]
                }))
                .unwrap();

                let in2 = QueryPlannerInput {
                    user_query: "My GitHub Issues",
                    past_steps: &[QueryStepOutput {
                        query: "My GitHub Issues".to_string(),
                        data: serde_json::from_value(json!({
                            "GitHub": {
                                "Issues": {
                                    "assumptions_made_during_query": "Assumed \"My GitHub Issues\" refers to issues assigned to the user",
                                    "queried_data": [
                                        {
                                            "id": 10,
                                            "title": "Test"
                                        }
                                    ]
                                }
                            }
                        }))
                        .unwrap(),
                        explanation: "Simple single-step query that matches the users exact query"
                            .to_string(),
                    }],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;

                let r_out2 = r_out2.as_return().context("to_return")?;
                let check1 = return_planner_step_matches(
                    r_out2,
                    Some(
                        serde_json::from_value(json!({
                            "0": {
                                "GitHub": [
                                    "Issues"
                                ]
                            }
                        }))
                        .unwrap(),
                    ),
                    None,
                    Confidence::Certain,
                    None,
                );

                let check2 = r_out2.user_message.as_ref().is_some_and(|message| message.to_lowercase().contains("assigned"));
                Ok(vec![check1, check2])
            })
        })
        .await;

        test.start_test_sequence("assumptions test - My Issues", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": [
                        "Issues",
                        "PullRequests",
                        "Repositories",
                        "Comments"
                    ],
                    "Jira": [
                        "Issues"
                    ]
                }))
                .unwrap();

                let in2 = QueryPlannerInput {
                    user_query: "My Issues",
                    past_steps: &[QueryStepOutput {
                        query: "My Issues".to_string(),
                        data: serde_json::from_value(json!({
                            "GitHub": {
                                "Issues": {
                                    "assumptions_made_during_query": "Assumed \"My Issue\" refers to issues assigned to the user",
                                    "queried_data": [
                                        {
                                            "id": 10,
                                            "title": "Test"
                                        }
                                    ]
                                }
                            },
                            "Jira": {
                                "Issues": {
                                    "assumptions_made_during_query": "Assumed \"My Issues\" refers to issues created by the user",
                                    "queried_data": [
                                        {
                                            "id": 2,
                                            "title": "Test2"
                                        }
                                    ]
                                }
                            }
                        }))
                        .unwrap(),
                        explanation: "Query both Jira and GitHub for the Issues"
                            .to_string(),
                    }],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;

                let r_out2 = r_out2.as_return().context("to_return")?;
                let check1 = return_planner_step_matches(
                    r_out2,
                    Some(
                        serde_json::from_value(json!({
                            "0": {
                                "GitHub": [
                                    "Issues"
                                ],
                                "Jira": [
                                    "Issues"
                                ]
                            }
                        }))
                        .unwrap(),
                    ),
                    None,
                    Confidence::Optimistic,
                    None,
                );

                let check2 = r_out2.user_message.as_ref().is_some_and(|message| message.to_lowercase().contains("assigned") && message.to_lowercase().contains("created"));
                Ok(vec![check1, check2])
            })
        })
        .await;

        test.start_test_sequence("assumptions test - My Issues from my last sprint", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": [
                        "Issues",
                        "PullRequests",
                        "Repositories",
                        "Comments"
                    ],
                    "Jira": [
                        "Sprints"
                    ]
                }))
                .unwrap();

                let in2 = QueryPlannerInput {
                    user_query: "My Issues from my last sprint",
                    past_steps: &[
                    QueryStepOutput {
                        query: "My last sprint dates".to_string(),
                        data: serde_json::from_value(json!({
                            "Jira": {
                                "Sprints": {
                                    "assumptions_made_during_query": "Assumed \"My last sprint\" is the last sprint the user participated in",
                                    "queried_data": [
                                        {
                                            "id": 10,
                                            "title": "Sprint #10",
                                            "start_date": "August 23rd, 2024",
                                            "end_date": "August 30th, 2024"
                                        }
                                    ]
                                }
                            }
                        })).unwrap(),
                        explanation: "Qeury Jira Sprints for the last sprinte dates".to_string(),
                    },
                    QueryStepOutput {
                        query: "My Issues between August 23rd 2024 and August 30th, 2024".to_string(),
                        data: serde_json::from_value(json!({
                            "GitHub": {
                                "Issues": {
                                    "assumptions_made_during_query": "Assumed \"My Issue\" refers to issues assigned to the user and issues created between August 23rd 2024 and August 30th 2024",
                                    "queried_data": [
                                        {
                                            "id": 10,
                                            "title": "Test"
                                        }
                                    ]
                                }
                            }
                        }))
                        .unwrap(),
                        explanation: "Query for GitHub Issues between the last sprint dates"
                            .to_string(),
                    }],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;

                let r_out2 = r_out2.as_return().context("to_return")?; 
                let check1 = return_planner_step_matches(
                    r_out2,
                    Some(
                        serde_json::from_value(json!({
                            "1": {
                                "GitHub": [
                                    "Issues"
                                ]
                            }
                        }))
                        .unwrap(),
                    ),
                    None,
                    Confidence::Optimistic,
                    None,
                );

                let check2 = r_out2.user_message.as_ref().is_some_and(|message| message.to_lowercase().contains("assigned") && message.to_lowercase().contains("sprint"));
                Ok(vec![check1, check2])
            })
        })
        .await;

        test
    }

    async fn score_planner_model() -> TestResult {
        let mut test = TestResult::new("planner");

        test.start_test_sequence("single step planner queries - open PRs", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": [
                        "Issues",
                        "PullRequests",
                        "Repositories",
                        "Comments"
                    ]
                }))
                .unwrap();

                let in1 = QueryPlannerInput {
                    user_query: "All open GitHub pull requests",
                    past_steps: &[],
                    available_sources: &sources,
                };
                let r_out1: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in1)
                    .await?;
                let check1 = query_planner_step_matches(
                    r_out1.as_query().context("to_query")?,
                    "All open GitHub pull requests",
                    serde_json::from_value(json!({
                        "GitHub": ["PullRequests"]
                    }))
                    .unwrap(),
                    Confidence::Certain,
                );

                let in2 = QueryPlannerInput {
                    user_query: "All open GitHub pull requests",
                    past_steps: &[QueryStepOutput {
                        query: "All open GitHub pull requests".to_string(),
                        data: serde_json::from_value(json!({
                            "GitHub": {
                                "PullRequests": {
                                    "assumptions_made_during_query": "",
                                    "queried_data": [
                                        {
                                            "id": 10,
                                            "title": "Test"
                                        }
                                    ]
                                }
                            }
                        }))
                        .unwrap(),
                        explanation: "Simple single-step query that matches the users exact query"
                            .to_string(),
                    }],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;
                let check2 = return_planner_step_matches(
                    r_out2.as_return().context("to_return")?,
                    Some(
                        serde_json::from_value(json!({
                            "0": {
                                "GitHub": [
                                    "PullRequests"
                                ]
                            }
                        }))
                        .unwrap(),
                    ),
                    None,
                    Confidence::Certain,
                    None,
                );

                Ok(vec![check1, check2])
            })
        })
        .await;

        test.start_test_sequence("single step planner queries - repos", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": [
                        "Issues",
                        "PullRequests",
                        "Repositories",
                        "Comments"
                    ]
                }))
                .unwrap();

                let in1 = QueryPlannerInput {
                    user_query: "My GitHub repositories",
                    past_steps: &[],
                    available_sources: &sources,
                };
                let r_out1: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in1)
                    .await?;
                let check1 = query_planner_step_matches(
                    r_out1.as_query().context("to_query")?,
                    "My GitHub repositories",
                    serde_json::from_value(json!({
                        "GitHub": ["Repositories"]
                    }))
                    .unwrap(),
                    Confidence::Certain,
                );

                let in2 = QueryPlannerInput {
                    user_query: "My GitHub repositories",
                    past_steps: &[QueryStepOutput {
                        query: "My GitHub repositories".to_string(),
                        data: serde_json::from_value(json!({
                            "GitHub": {
                                "Repositories": {
                                    "assumptions_made_during_query": "",
                                    "queried_data": [
                                        {
                                            "id": 10,
                                            "title": "Test"
                                        }
                                    ]
                                }
                            }
                        }))
                        .unwrap(),
                        explanation: "Simple single-step query".to_string(),
                    }],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;
                let check2 = return_planner_step_matches(
                    r_out2.as_return().context("to_return")?,
                    Some(
                        serde_json::from_value(json!({
                            "0": {
                                "GitHub": [
                                    "Repositories"
                                ]
                            }
                        }))
                        .unwrap(),
                    ),
                    None,
                    Confidence::Certain,
                    None,
                );

                Ok(vec![check1, check2])
            })
        })
        .await;

        test.start_test_sequence(
            "multi-step multi-data planner - sprints and issues",
            3,
            move || {
                Box::pin(async move {
                    let sources: Sources = serde_json::from_value(json!({
                        "GitHub": ["Issues", "PullRequests", "Repositories", "Comments"],
                        "Jira": ["Issues", "Sprints"]
                    }))
                    .unwrap();

                    // Step 1: The user asks for "Issues from my last sprint"
                    // This should result in a multi-step plan:
                    // First query: "My last sprint dates"
                    let in1 = QueryPlannerInput {
                        user_query: "Issues from my last sprint",
                        past_steps: &[],
                        available_sources: &sources,
                    };
                    let r_out1: Step = MODELS
                        .transform
                        .chat_with_value(PLANNER_PROMPT, &in1)
                        .await?;

                    // Validate that we got a query for "My last sprint dates" (the first
                    // decomposition step)
                    let check1 = query_planner_step_matches_range(
                        r_out1.as_query().context("to_query")?,
                        10..50,
                        serde_json::from_value(json!({
                            "Jira": ["Sprints"]
                        }))
                        .unwrap(),
                        Confidence::Optimistic,
                    );

                    // Step 2: Now we simulate receiving the sprint data and query for issues in
                    // that range
                    let in2 = QueryPlannerInput {
                        user_query: "Issues from my last sprint",
                        past_steps: &[QueryStepOutput {
                            query: "My last sprint dates".to_string(),
                            data: serde_json::from_value(json!({
                                "Jira": {
                                    "Sprints": {
                                        "assumptions_made_during_query": "",
                                        "queried_data": [
                                            {
                                                "name": "Sprint 45",
                                                "start_date": "2023-05-01",
                                                "end_date": "2023-05-14"
                                            }
                                        ]
                                    }
                                }
                            }))
                            .unwrap(),
                            explanation: "First get last sprint dates".to_string(),
                        }],
                        available_sources: &sources,
                    };
                    let r_out2: Step = MODELS
                        .transform
                        .chat_with_value(PLANNER_PROMPT, &in2)
                        .await?;

                    // Validate that now it queries for issues in the given date range
                    let check2 = query_planner_step_matches_range(
                        r_out2.as_query().context("to_query")?,
                        25..50,
                        serde_json::from_value(json!({
                            "GitHub": ["Issues"],
                            "Jira": ["Issues"]
                        }))
                        .unwrap(),
                        Confidence::Optimistic,
                    );

                    // Step 3: Simulate receiving the issues data and now we return it
                    let in3 = QueryPlannerInput {
                        user_query: "Issues from my last sprint",
                        past_steps: &[
                            QueryStepOutput {
                                query: "My last sprint dates".to_string(),
                                data: serde_json::from_value(json!({
                                    "Jira": {
                                        "Sprints": {
                                            "assumptions_made_during_query": "",
                                            "queried_data": [
                                                {
                                                    "name": "Sprint 45",
                                                    "start_date": "2023-05-01",
                                                    "end_date": "2023-05-14"
                                                }
                                            ]
                                        }
                                    }
                                }))
                                .unwrap(),
                                explanation: "First get last sprint dates".to_string(),
                            },
                            QueryStepOutput {
                                query: "Issues between 2023-05-01 and 2023-05-14".to_string(),
                                data: serde_json::from_value(json!({
                                    "GitHub": {
                                        "Issues": {
                                            "assumptions_made_during_query": "",
                                            "queried_data": [
                                                {"id": 1, "title": "Bug fix"},
                                                {"id": 2, "title": "Feature request"}
                                            ]
                                        }
                                    },
                                    "Jira": {
                                        "Issues": {
                                            "assumptions_made_during_query": "",
                                            "queried_data": [
                                                {"id": "JIRA-100", "title": "UI improvement"}
                                            ]
                                        }
                                    }
                                }))
                                .unwrap(),
                                explanation:
                                    "Now that we have the sprint dates, use them to get the issues"
                                        .to_string(),
                            },
                        ],
                        available_sources: &sources,
                    };
                    let r_out3: Step = MODELS
                        .transform
                        .chat_with_value(PLANNER_PROMPT, &in3)
                        .await?;

                    // Validate the return step
                    // Since we have assumptions from the first step, we must include user_message.
                    // Also verify that it returns data from step 1 (0-based indexing: step 1 is
                    // where issues came from).
                    let check3 = return_planner_step_matches(
                        r_out3.as_return().context("to_return")?,
                        Some(
                            serde_json::from_value(json!({
                                "1": {
                                    "GitHub": ["Issues"],
                                    "Jira": ["Issues"]
                                }
                            }))
                            .unwrap(),
                        ),
                        None,
                        Confidence::Optimistic,
                        None,
                    );

                    Ok(vec![check1, check2, check3])
                })
            },
        )
        .await;

        test.start_test_sequence("single-step multi-source latest issue selection", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": ["Issues", "PullRequests", "Repositories", "Comments"],
                    "Jira": ["Issues", "Sprints"]
                }))
                .unwrap();

                // Step 1: The user asks for "My last issue"
                // The planner should query both GitHub and Jira issues.
                let in1 = QueryPlannerInput {
                    user_query: "My last issue",
                    past_steps: &[],
                    available_sources: &sources,
                };
                let r_out1: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in1)
                    .await?;

                // Validate that the first output is a query.
                // The exact query is "My last issue". We want both GitHub and Jira issues targeted.
                let check1 = query_planner_step_matches(
                    r_out1.as_query().context("to_query")?,
                    "My last issue",
                    serde_json::from_value(json!({
                        "GitHub": ["Issues"],
                        "Jira": ["Issues"]
                    })).unwrap(),
                    Confidence::Optimistic,
                );

                // Step 2: Simulate receiving data from both GitHub and Jira, each with assumptions.
                // The Jira issue is newer.
                let in2 = QueryPlannerInput {
                    user_query: "My last issue",
                    past_steps: &[QueryStepOutput {
                        query: "My last issue".to_string(),
                        data: serde_json::from_value(json!({
                            "GitHub": {
                                "Issues": {
                                    "assumptions_made_during_query": "Assuming user is the assignee",
                                    "queried_data": [
                                        {
                                            "id": 100,
                                            "title": "Fix login bug",
                                            "created_at": "2023-05-10T09:00:00Z"
                                        }
                                    ]
                                }
                            },
                            "Jira": {
                                "Issues": {
                                    "assumptions_made_during_query": "Assuming priority field absent means normal priority",
                                    "queried_data": [
                                        {
                                            "id": "JIRA-200",
                                            "title": "Update payment flow",
                                            "created_at": "2023-05-11T10:30:00Z"
                                        }
                                    ]
                                }
                            }
                        }))
                        .unwrap(),
                        explanation: "Get issues from both GitHub and Jira before selecting the one with most recent one".to_string(),
                    }],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;

                // Validate the return step.
                // The Jira issue is newer, so we should return only the Jira issue.
                // user_message must reflect assumptions from previous steps (both from GitHub and Jira).
                let check2 = return_planner_step_matches(
                    r_out2.as_return().context("to_return")?,
                    Some(
                        serde_json::from_value(json!({
                            "0": {
                                "Jira": ["Issues"]
                            }
                        }))
                        .unwrap(),
                    ),
                    None,
                    Confidence::Optimistic,
                    Some(Some(0..200)) // user_message required due to assumptions from the first step
                );

                Ok(vec![check1, check2])
            })
        }).await;

        test.start_test_sequence(
            "multi-step query - comments on last GitHub PR",
            4,
            move || {
                Box::pin(async move {
                    let sources: Sources = serde_json::from_value(json!({
                        "GitHub": ["Issues", "PullRequests", "Repositories", "Comments"]
                    }))
                    .unwrap();

                    // Step 1: The user asks for "comments on my last GitHub pull request"
                    // The planner should first query "My last GitHub pull request" since it's a
                    // multi-step query.
                    let in1 = QueryPlannerInput {
                        user_query: "comments on my last GitHub pull request",
                        past_steps: &[],
                        available_sources: &sources,
                    };
                    let r_out1: Step = MODELS
                        .transform
                        .chat_with_value(PLANNER_PROMPT, &in1)
                        .await?;

                    // Validate the first query targets GitHub PullRequests
                    let targets_step1: serde_json::Value = json!({
                        "GitHub": ["PullRequests"]
                    });
                    let check1 = query_planner_step_matches_range(
                        r_out1.as_query().context("to_query")?,
                        10..50,
                        serde_json::from_value(targets_step1).unwrap(),
                        Confidence::Optimistic,
                    );

                    // Step 2: Simulate receiving the pull request data from GitHub
                    let in2 = QueryPlannerInput {
                        user_query: "comments on my last GitHub pull request",
                        past_steps: &[QueryStepOutput {
                            query: "My last GitHub pull request".to_string(),
                            data: serde_json::from_value(json!({
                                "GitHub": {
                                    "PullRequests": {
                                        "assumptions_made_during_query": "",
                                        "queried_data": [
                                            {
                                                "id": 123,
                                                "title": "Fix payment bug",
                                                "created_at": "2023-05-10T09:00:00Z",
                                                "repository": "SilasMarvin/test"
                                            }
                                        ]
                                    }
                                }
                            }))
                            .unwrap(),
                            explanation: "First get the last GitHub pull request".to_string(),
                        }],
                        available_sources: &sources,
                    };
                    let r_out2: Step = MODELS
                        .transform
                        .chat_with_value(PLANNER_PROMPT, &in2)
                        .await?;

                    // Now it should query "comments on:\n${summarize($0)}" against GitHub Comments
                    let targets_step2: serde_json::Value = json!({
                        "GitHub": ["Comments"]
                    });
                    let check2 = query_planner_step_matches_range(
                        r_out2.as_query().context("to_query")?,
                        30..100,
                        serde_json::from_value(targets_step2).unwrap(),
                        Confidence::Optimistic,
                    );
                    // Ensure it uses summarize properly
                    let check3 = r_out2
                        .as_query()
                        .context("to_query")?
                        .query
                        .contains("summarize");

                    // Step 3: Simulate receiving the comments data and now we return it
                    let in3 = QueryPlannerInput {
                        user_query: "comments on my last GitHub pull request",
                        past_steps: &[
                            QueryStepOutput {
                                query: "My last GitHub pull request".to_string(),
                                data: serde_json::from_value(json!({
                                    "GitHub": {
                                        "PullRequests": {
                                            "assumptions_made_during_query": "",
                                            "queried_data": [
                                                {
                                                    "id": 123,
                                                    "title": "Fix payment bug",
                                                    "created_at": "2023-05-10T09:00:00Z",
                                                    "repository": "SilasMarvin/test"
                                                }
                                            ]
                                        }
                                    }
                                }))
                                .unwrap(),
                                explanation: "First get the last GitHub pull request".to_string(),
                            },
                            QueryStepOutput {
                                query: "comments on:\n${summarize($0)}".to_string(),
                                data: serde_json::from_value(json!({
                                    "GitHub": {
                                        "Comments": {
                                            "assumptions_made_during_query": "",
                                            "queried_data": [
                                                {"id": 999, "body": "LGTM!"},
                                                {"id": 1000, "body": "Needs more tests."}
                                            ]
                                        }
                                    }
                                }))
                                .unwrap(),
                                explanation: "Comments on the last GitHub pull request".to_string(),
                            },
                        ],
                        available_sources: &sources,
                    };
                    let r_out3: Step = MODELS
                        .transform
                        .chat_with_value(PLANNER_PROMPT, &in3)
                        .await?;

                    // Validate the return step:
                    // Should return the data from step 1 (index 1) GitHub Comments.
                    // No assumptions made, so no user_message is required.
                    let data_selection: serde_json::Value = json!({
                        "1": {
                            "GitHub": ["Comments"]
                        }
                    });
                    let check4 = return_planner_step_matches(
                        r_out3.as_return().context("to_return")?,
                        Some(serde_json::from_value(data_selection).unwrap()),
                        None,
                        Confidence::Optimistic,
                        None,
                    );

                    Ok(vec![check1, check2, check3, check4])
                })
            },
        )
        .await;

        test.start_test_sequence("multi-step query - Slack messages about issues from my last sprint", 4, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": ["Issues", "PullRequests", "Repositories", "Comments"],
                    "Jira": ["Issues", "Sprints"],
                    "Slack": ["Messages"]
                }))
                .unwrap();

                // Step 1: The user asks "Slack messages about issues from my last sprint"
                // The model should first resolve "my last sprint dates".
                let in1 = QueryPlannerInput {
                    user_query: "Slack messages about issues from my last sprint",
                    past_steps: &[],
                    available_sources: &sources,
                };
                let r_out1: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in1)
                    .await?;

                // Validate step 1: should query "My last sprint dates" from Jira Sprints
                let check1 = query_planner_step_matches_range(
                    r_out1.as_query().context("to_query")?,
                    10..100,
                    serde_json::from_value(json!({
                        "Jira": ["Sprints"]
                    })).unwrap(),
                    Confidence::Optimistic,
                );

                // Step 2: Simulate receiving the sprint data from Jira
                // We provide the explanation from step 1 and the sprint data retrieved.
                let in2 = QueryPlannerInput {
                    user_query: "Slack messages about issues from my last sprint",
                    past_steps: &[
                        QueryStepOutput {
                            query: "My last sprint dates".to_string(),
                            data: serde_json::from_value(json!({
                                "Jira": {
                                    "Sprints": {
                                        "queried_data": [
                                            {
                                                "name": "Sprint 45",
                                                "start_date": "2023-05-01",
                                                "end_date": "2023-05-14"
                                            }
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "First, retrieve the dates for the last sprint from Jira to identify the relevant issues.".to_string(),
                        }
                    ],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;

                // Validate step 2: should now query issues based on the returned date range
                let check2 = query_planner_step_matches_range(
                    r_out2.as_query().context("to_query")?,
                    20..100,
                    serde_json::from_value(json!({
                        "GitHub": ["Issues"],
                        "Jira": ["Issues"]
                    })).unwrap(),
                    Confidence::Optimistic,
                );

                // Step 3: Simulate receiving the issues data from GitHub and Jira
                let in3 = QueryPlannerInput {
                    user_query: "Slack messages about issues from my last sprint",
                    past_steps: &[
                        QueryStepOutput {
                            query: "My last sprint dates".to_string(),
                            data: serde_json::from_value(json!({
                                "Jira": {
                                    "Sprints": {
                                        "queried_data": [
                                            {
                                                "name": "Sprint 45",
                                                "start_date": "2023-05-01",
                                                "end_date": "2023-05-14"
                                            }
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "First, retrieve the dates for the last sprint from Jira to identify the relevant issues.".to_string(),
                        },
                        QueryStepOutput {
                            query: "Issues between 2023-05-01 and 2023-05-14".to_string(),
                            data: serde_json::from_value(json!({
                                "GitHub": {
                                    "Issues": {
                                        "queried_data": [
                                            {"id": 1, "title": "Bug fix"},
                                            {"id": 2, "title": "Feature request"}
                                        ]
                                    }
                                },
                                "Jira": {
                                    "Issues": {
                                        "queried_data": [
                                            {"id": "JIRA-100", "title": "UI improvement"}
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "With the sprint dates known, retrieve issues from GitHub and Jira.".to_string(),
                        }
                    ],
                    available_sources: &sources,
                };
                let r_out3: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in3)
                    .await?;

                // Validate step 3: should now query Slack messages about the summarized issues
                let check3 = query_planner_step_matches_range(
                    r_out3.as_query().context("to_query")?,
                    25..200,
                    serde_json::from_value(json!({
                        "Slack": ["Messages"]
                    })).unwrap(),
                    Confidence::Optimistic,
                );

                // Step 4: Simulate receiving Slack messages referencing the issues
                let in4 = QueryPlannerInput {
                    user_query: "Slack messages about issues from my last sprint",
                    past_steps: &[
                        QueryStepOutput {
                            query: "My last sprint dates".to_string(),
                            data: serde_json::from_value(json!({
                                "Jira": {
                                    "Sprints": {
                                        "queried_data": [
                                            {
                                                "name": "Sprint 45",
                                                "start_date": "2023-05-01",
                                                "end_date": "2023-05-14"
                                            }
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Need to determine the last sprint date range first.".to_string(),
                        },
                        QueryStepOutput {
                            query: "Issues between 2023-05-01 and 2023-05-14".to_string(),
                            data: serde_json::from_value(json!({
                                "GitHub": {
                                    "Issues": {
                                        "queried_data": [
                                            {"id": 1, "title": "Bug fix"},
                                            {"id": 2, "title": "Feature request"}
                                        ]
                                    }
                                },
                                "Jira": {
                                    "Issues": {
                                        "queried_data": [
                                            {"id": "JIRA-100", "title": "UI improvement"}
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "With the sprint dates known, retrieve issues from GitHub and Jira.".to_string(),
                        },
                        QueryStepOutput {
                            query: "Slack messages about:\n${summarize($1)}".to_string(),
                            data: serde_json::from_value(json!({
                                "Slack": {
                                    "Messages": {
                                        "queried_data": [
                                            {"id": "MSG-1", "text": "We need to fix the UI on the payment page."},
                                            {"id": "MSG-2", "text": "The bug fix was deployed last week."}
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Now that issues are known, find Slack messages referencing them.".to_string(),
                        }
                    ],
                    available_sources: &sources,
                };
                let r_out4: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in4)
                    .await?;

                // Validate step 4: should return the Slack messages (from step 2 index is "2" because steps start at 0)
                let data_selection = serde_json::from_value(json!({
                    "2": {
                        "Slack": ["Messages"]
                    }
                })).unwrap();

                let check4 = return_planner_step_matches(
                    r_out4.as_return().context("to_return")?,
                    Some(data_selection),
                    None,
                    Confidence::Optimistic,
                    None
                );

                Ok(vec![check1, check2, check3, check4])
            })
        }).await;

        test.start_test_sequence("multi-step query - Slack messages about PRs referencing my last Jira issue", 6, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": ["Issues", "PullRequests", "Repositories", "Comments"],
                    "Jira": ["Issues", "Sprints"],
                    "Slack": ["Messages"]
                }))
                .unwrap();

                // User Query: "Slack messages about PRs referencing my last Jira issue"
                // Steps:
                // 1. Identify "my last Jira issue"
                // 2. Find "PRs referencing:\n${summarize($0)}" on GitHub
                // 3. Find "Slack messages about:\n${summarize($1)}"

                // Step 1: Resolve "my last Jira issue"
                let in1 = QueryPlannerInput {
                    user_query: "Slack messages about PRs referencing my last Jira issue",
                    past_steps: &[],
                    available_sources: &sources,
                };
                let r_out1: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in1)
                    .await?;

                let check1 = query_planner_step_matches_range(
                    r_out1.as_query().context("to_query")?,
                    10..50,
                    serde_json::from_value(json!({
                        "Jira": ["Issues"]
                    })).unwrap(),
                    Confidence::Optimistic,
                );

                // Step 2: Simulate receiving the last Jira issue data
                let in2 = QueryPlannerInput {
                    user_query: "Slack messages about PRs referencing my last Jira issue",
                    past_steps: &[
                        QueryStepOutput {
                            query: "My last Jira issue".to_string(),
                            data: serde_json::from_value(json!({
                                "Jira": {
                                    "Issues": {
                                        "queried_data": [
                                            {
                                                "id": "JIRA-100",
                                                "title": "Fix checkout bug",
                                                "created_at": "2023-05-01T10:00:00Z"
                                            }
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Need to find the last Jira issue first.".to_string(),
                        }
                    ],
                    available_sources: &sources,
                };
                let r_out2: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in2)
                    .await?;

                // Validate step 2: Query PRs referencing the Jira issue
                let check2 = query_planner_step_matches_range(
                    r_out2.as_query().context("to_query")?,
                    25..100,
                    serde_json::from_value(json!({
                        "GitHub": ["PullRequests"]
                    })).unwrap(),
                    Confidence::Optimistic,
                );
                // Make sure it uses summarize correctly
                let check3 = r_out2.as_query().context("to_query")?.query.contains("summarize");

                // Step 3: Simulate receiving the PRs referencing the Jira issue
                let in3 = QueryPlannerInput {
                    user_query: "Slack messages about PRs referencing my last Jira issue",
                    past_steps: &[
                        QueryStepOutput {
                            query: "My last Jira issue".to_string(),
                            data: serde_json::from_value(json!({
                                "Jira": {
                                    "Issues": {
                                        "queried_data": [
                                            {
                                                "id": "JIRA-100",
                                                "title": "Fix checkout bug",
                                                "created_at": "2023-05-01T10:00:00Z"
                                            }
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Need to find the last Jira issue first.".to_string(),
                        },
                        QueryStepOutput {
                            query: "PRs referencing:\n${summarize($0)}".to_string(),
                            data: serde_json::from_value(json!({
                                "GitHub": {
                                    "PullRequests": {
                                        "queried_data": [
                                            {"id": 500, "title": "Add checkout logging"},
                                            {"id": 501, "title": "Fix pricing calculation"}
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Now find PRs referencing the identified Jira issue.".to_string(),
                        }
                    ],
                    available_sources: &sources,
                };
                let r_out3: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in3)
                    .await?;

                // Validate step 3: Query Slack messages about the PRs
                let check4 = query_planner_step_matches_range(
                    r_out3.as_query().context("to_query")?,
                    25..100,
                    serde_json::from_value(json!({
                        "Slack": ["Messages"]
                    })).unwrap(),
                    Confidence::Optimistic,
                );
                // Make sure it uses summarize correctly
                let check5 = r_out3.as_query().context("to_query")?.query.contains("summarize");

                // Step 4: Simulate receiving Slack messages referencing the PRs
                let in4 = QueryPlannerInput {
                    user_query: "Slack messages about PRs referencing my last Jira issue",
                    past_steps: &[
                        QueryStepOutput {
                            query: "My last Jira issue".to_string(),
                            data: serde_json::from_value(json!({
                                "Jira": {
                                    "Issues": {
                                        "queried_data": [
                                            {
                                                "id": "JIRA-100",
                                                "title": "Fix checkout bug",
                                                "created_at": "2023-05-01T10:00:00Z"
                                            }
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Need to find the last Jira issue first.".to_string(),
                        },
                        QueryStepOutput {
                            query: "PRs referencing:\n${summarize($0)}".to_string(),
                            data: serde_json::from_value(json!({
                                "GitHub": {
                                    "PullRequests": {
                                        "queried_data": [
                                            {"id": 500, "title": "Add checkout logging"},
                                            {"id": 501, "title": "Fix pricing calculation"}
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Now find PRs referencing the identified Jira issue.".to_string(),
                        },
                        QueryStepOutput {
                            query: "Slack messages about:\n${summarize($1)}".to_string(),
                            data: serde_json::from_value(json!({
                                "Slack": {
                                    "Messages": {
                                        "queried_data": [
                                            {"id": "MSG-10", "text": "The PR for checkout bug is merged."},
                                            {"id": "MSG-11", "text": "Need to review PR #501 before release."}
                                        ]
                                    }
                                }
                            })).unwrap(),
                            explanation: "Now retrieve Slack messages referencing these PRs.".to_string(),
                        }
                    ],
                    available_sources: &sources,
                };
                let r_out4: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in4)
                    .await?;

                // Validate the return step:
                // Return the Slack messages from step 2 (the third step of data retrieval, index 2).
                let data_selection = serde_json::from_value(json!({
                    "2": {
                        "Slack": ["Messages"]
                    }
                })).unwrap();

                let check6 = return_planner_step_matches(
                    r_out4.as_return().context("to_return")?,
                    Some(data_selection),
                    None,
                    Confidence::Optimistic,
                    None
                );

                Ok(vec![check1, check2, check3, check4, check5, check6])
            })
        }).await;

        // A general knowledge question that should return an error immediately
        test.start_test_sequence("general knowledge question - no query", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": ["Issues", "PullRequests"],
                    "Slack": ["Messages"]
                }))
                .unwrap();

                let in1 = QueryPlannerInput {
                    user_query: "Who discovered electricity?",
                    past_steps: &[],
                    available_sources: &sources,
                };

                let r_out1: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in1)
                    .await?;

                let return_step = r_out1.as_return().context("to_return")?;

                Ok(vec![
                    return_step.user_message.is_some(),
                    return_step.confidence == Confidence::Certain,
                ])
            })
        })
        .await;

        // A nonsensical request that can't be fulfilled by any data source
        test.start_test_sequence("nonsensical request - no query", 2, move || {
            Box::pin(async move {
                let sources: Sources = serde_json::from_value(json!({
                    "GitHub": ["Issues", "PullRequests"],
                    "Jira": ["Issues", "Sprints"]
                }))
                .unwrap();

                let in1 = QueryPlannerInput {
                    user_query: "Find a unicorn made of chocolate",
                    past_steps: &[],
                    available_sources: &sources,
                };

                let r_out1: Step = MODELS
                    .transform
                    .chat_with_value(PLANNER_PROMPT, &in1)
                    .await?;

                let return_step = r_out1.as_return().context("to_return")?;

                Ok(vec![
                    return_step.user_message.is_some(),
                    return_step.confidence == Confidence::Certain,
                ])
            })
        })
        .await;

        test
    }

    async fn score_google_docs_model() -> TestResult {
        let mut test = TestResult::new("google docs model");

        // test.start_test_sequence("nonsensical request - no query", 2, move || {
        //     Box::pin(async move {
        //         let sources: Sources = serde_json::from_value(json!({
        //             "Google": ["Docs"],
        //         }))
        //         .unwrap();

        //         // let in1 = QueryPlannerInput {
        //         //     user_query: "My last project overview",
        //         //     past_steps: &[],
        //         //     available_sources: &sources,
        //         // };

        //         // let r_out1: Step = MODELS
        //         //     .transform
        //         //     .chat_with_value(PLANNER_PROMPT, &in1)
        //         //     .await?;

        //         // let return_step = r_out1.as_return().context("to_return")?;

        //         // Ok(vec![
        //         //     return_step.user_message.is_some(),
        //         //     return_step.confidence == Confidence::Certain,
        //         // ])

        //         todo!()
        //     })
        // })
        // .await;

        test
    }

    async fn score_no_matching_data_sources(port: u16) -> TestResult {
        let mut test = TestResult::new("no_matching_data_sources");

        // A reasonable query that has no matching data sources
        //
        // This test verifies:
        // - Proper erroring when asking for data from sources that don't exist
        test.start_test_sequence("'what are my open Jira tickets'", 2, move || {
            Box::pin(async move {
                let response = get_context("what are my open Jira Tickets", port, CannedUser::Dev1)
                    .await?
                    .to_failure()
                    .context("to_success")?;

                Ok(vec![
                    response.confidence == Confidence::Certain,
                    response.user_message.len() > 10 && response.user_message.len() < 200,
                ])
            })
        })
        .await;

        // A reasonable query that has no matching data sources
        //
        // This test verifies:
        // - Proper erroring when asking for data from sources that don't exist
        test.start_test_sequence("'what was my last email'", 2, move || {
            Box::pin(async move {
                let response = get_context("what was my last email", port, CannedUser::Dev1)
                    .await?
                    .to_failure()
                    .context("to_success")?;

                Ok(vec![
                    response.confidence == Confidence::Certain,
                    response.user_message.len() > 10 && response.user_message.len() < 200,
                ])
            })
        })
        .await;

        test
    }

    async fn score_ridiculous_requests(port: u16) -> TestResult {
        let mut test = TestResult::new("ridiculous_requests");

        // A ridiculous query that should error immediatly
        //
        // This test verifies:
        // - Proper erroring when asked questions we shouldn't answer
        test.start_test_sequence("'what is the capital of Peru'", 2, move || {
            Box::pin(async move {
                let response = get_context("what is the capital of Peru", port, CannedUser::Dev1)
                    .await?
                    .to_failure()
                    .context("to_success")?;

                Ok(vec![
                    response.confidence == Confidence::Certain,
                    response.user_message.len() > 10 && response.user_message.len() < 200,
                ])
            })
        })
        .await;

        // A ridiculous query that should error immediatly
        //
        // This test verifies:
        // - Proper erroring when asked questions we shouldn't answer
        test.start_test_sequence("'what is GitHub's market share'", 2, move || {
            Box::pin(async move {
                let response = get_context("what is GitHub's market share", port, CannedUser::Dev1)
                    .await?
                    .to_failure()
                    .context("to_success")?;

                Ok(vec![
                    response.confidence == Confidence::Certain,
                    response.user_message.len() > 10 && response.user_message.len() < 200,
                ])
            })
        })
        .await;

        test
    }

    async fn score_slack_model() -> TestResult {
        let mut test = TestResult::new("slack model");

        test.start_test_sequence(
            "single step queries - My last message",
            4,
            move || {
                Box::pin(async move {
                    let in1 = SlackQueryInput {
                        query: "My last message",
                        user_id: "U123AB",
                        past_steps: &[],
                    };
                    let r_out1: SlackStep =
                        MODELS.transform.chat_with_value(SLACK_PROMPT, &in1).await?;

                    let query = r_out1.as_query().context("to_query")?;
                    let check1 = query.confidence == Confidence::Certain;
                    let check2 = query.sql_query.contains(
                        "FROM slack.messages WHERE user = 'U123AB' ORDER BY ts DESC LIMIT 1",
                    );

                    let in2 = SlackQueryInput {
                        query: "My last message",
                        user_id: "U123AB",
                        past_steps: &[SlackQueryWithData {
                            sql_query: "SELECT JSON_BUILD_OBJECT('id', id, 'channel_id', channel_id, 'user', user, 'text', text, 'ts', ts) AS message FROM slack.messages WHERE user = 'U123AB' ORDER BY ts DESC LIMIT 1;".to_string(),
                            confidence: Confidence::Certain,
                            explanation: "Retrieve the most recent message sent by the user, ordered by timestamp.".to_string(),
                            data: vec![serde_json::json!({
                                "id": "U123ABABAB",
                                "channel_id": "U123ABABC",
                                "user": "U123AB",
                                "ts": "1720804777.854039"
                            })]
                        }],

                    };
                    let r_out2: SlackStep =
                        MODELS.transform.chat_with_value(SLACK_PROMPT, &in2).await?;

                    let ret = r_out2.as_return().context("to_return")?;
                    let check3 = ret.confidence == Confidence::Certain;
                    let check4 = ret.data_selection == Some(vec![0]);

                    Ok(vec![check1, check2, check3, check4])
                })
            },
        )
        .await;

        test.start_test_sequence(
            "multi-step queries - The last conversation about tomatoes in general",
            6,
            move || {
                Box::pin(async move {
                    // Step 1: Identify the last message in #general mentioning tomatoes
                    let in1 = SlackQueryInput {
                        query: "The last conversation about tomatoes in general",
                        user_id: "U123AB",
                        past_steps: &[],
                    };
                    let r_out1: SlackStep =
                        MODELS.transform.chat_with_value(SLACK_PROMPT, &in1).await?;

                    let query = r_out1.as_query().context("to_query")?;
                    let check1 = query.confidence >= Confidence::Optimistic;
                    let check2 = query.sql_query.contains(
                        "FROM slack.messages WHERE channel_id = (SELECT id FROM slack.channels WHERE name = 'general') AND text_tsv @@ to_tsquery('english', 'tomato') ORDER BY ts DESC LIMIT 1",
                    ) || query.sql_query.contains(
                        "FROM slack.messages WHERE channel_id = (SELECT id FROM slack.channels WHERE name = 'general') AND text_tsv @@ to_tsquery('english', 'tomatoes') ORDER BY ts DESC LIMIT 1",
                    );

                    // Step 2: Use the thread_ts or ts of the identified message to fetch the conversation
                    let in2 = SlackQueryInput {
                        query: "The last conversation about tomatoes in general",
                        user_id: "U123AB",
                        past_steps: &[SlackQueryWithData {
                            sql_query: "SELECT JSON_BUILD_OBJECT('id', id, 'channel_id', channel_id, 'type', type, 'user', user, 'text', text, 'thread_ts', thread_ts, 'ts', ts) AS message FROM slack.messages WHERE channel_id = (SELECT id FROM slack.channels WHERE name = 'general') AND text_tsv @@ to_tsquery('english', 'tomatoes') ORDER BY ts DESC LIMIT 1;".to_string(),
                            confidence: Confidence::Certain,
                            explanation: "Identify the last message in #general mentioning tomatoes.".to_string(),
                            data: vec![serde_json::json!({
                                "id": "M12345",
                                "channel_id": "C12345",
                                "type": "message",
                                "user": "U123AB",
                                "text": "Tomatoes are great for the next project.",
                                "thread_ts": "1720804700.854039",
                                "ts": "1720804700.854039"
                            })]
                        }],
                    };
                    let r_out2: SlackStep =
                        MODELS.transform.chat_with_value(SLACK_PROMPT, &in2).await?;

                    let query = r_out2.as_query().context("to_query")?;
                    let check3 = query.confidence >= Confidence::Optimistic;
                    let check4 = query.sql_query.contains("FROM slack.messages") && query.sql_query.contains("thread_ts = '1720804700.854039'") && query.sql_query.contains("ORDER BY ts ASC");

                    // Step 3: Return the conversation
                    let in3 = SlackQueryInput {
                        query: "The last conversation about tomatoes in general",
                        user_id: "U123AB",
                        past_steps: &[
                            SlackQueryWithData {
                                sql_query: "SELECT JSON_BUILD_OBJECT('id', id, 'channel_id', channel_id, 'type', type, 'user', user, 'text', text, 'thread_ts', thread_ts, 'ts', ts) AS message FROM slack.messages WHERE channel_id = (SELECT id FROM slack.channels WHERE name = 'general') AND text_tsv @@ to_tsquery('english', 'tomatoes') ORDER BY ts DESC LIMIT 1;".to_string(),
                                confidence: Confidence::Certain,
                                explanation: "Identify the last message in #general mentioning tomatoes.".to_string(),
                                data: vec![serde_json::json!({
                                    "id": "M12345",
                                    "channel_id": "C12345",
                                    "type": "message",
                                    "user": "U123AB",
                                    "text": "Tomatoes are great for the next project.",
                                    "thread_ts": "1720804700.854039",
                                    "ts": "1720804700.854039"
                                })]
                            },
                            SlackQueryWithData {
                                sql_query: "SELECT JSON_BUILD_OBJECT('id', id, 'channel_id', channel_id, 'type', type, 'user', user, 'text', text, 'thread_ts', thread_ts, 'ts', ts) AS message FROM slack.messages WHERE channel_id = 'C12345' AND (thread_ts = '1720804700.854039' OR ts >= '1720804700.854039') ORDER BY ts ASC LIMIT 10;".to_string(),
                                confidence: Confidence::Certain,
                                explanation: "Retrieve the conversation thread or messages following the identified message about tomatoes.".to_string(),
                                data: vec![serde_json::json!({
                                    "id": "M12346",
                                    "channel_id": "C12345",
                                    "type": "message",
                                    "user": "U67890",
                                    "text": "Yes, tomatoes are a great idea!",
                                    "thread_ts": "1720804700.854039",
                                    "ts": "1720804800.854039"
                                })]
                            }
                        ],
                    };
                    let r_out3: SlackStep =
                        MODELS.transform.chat_with_value(SLACK_PROMPT, &in3).await?;

                    let ret = r_out3.as_return().context("to_return")?;
                    let check5 = ret.confidence == Confidence::Certain;
                    let check6 = ret.data_selection == Some(vec![1]) || ret.data_selection == Some(vec![0, 1]);

                    Ok(vec![check1, check2, check3, check4, check5, check6])
                })
            },
        )
        .await;

        test.start_test_sequence(
            "clarification required - Show me everything about the project",
            2,
            move || {
                Box::pin(async move {
                    // Step 1: Ambiguous query
                    let in1 = SlackQueryInput {
                        query: "Show me everything about the project",
                        user_id: "U123AB",
                        past_steps: &[],
                    };
                    let r_out1: SlackStep =
                        MODELS.transform.chat_with_value(SLACK_PROMPT, &in1).await?;

                    let ret = r_out1.as_return().context("to_return")?;
                    let check1 = ret.confidence == Confidence::Doubtful;
                    let check2 = ret
                        .user_message
                        .as_ref()
                        .is_some_and(|message| message.len() > 10);

                    Ok(vec![check1, check2])
                })
            },
        )
        .await;

        test.start_test_sequence(
            "unsupported query - Who reacted to my last message",
            1,
            move || {
                Box::pin(async move {
                    let in1 = SlackQueryInput {
                        query: "Who reacted to my last message?",
                        user_id: "U123AB",
                        past_steps: &[],
                    };
                    let r_out1: SlackStep =
                        MODELS.transform.chat_with_value(SLACK_PROMPT, &in1).await?;

                    let ret = r_out1.as_return().context("to_return")?;
                    let check1 = ret
                        .user_message
                        .as_ref()
                        .is_some_and(|message| message.len() > 10);

                    Ok(vec![check1])
                })
            },
        )
        .await;

        test
    }

    async fn score_slack(port: u16) -> TestResult {
        let mut test = TestResult::new("slack end->end");

        // Simple two step query to get last message
        //
        // This test verifies:
        // - Simple two step plan
        test.start_test_sequence("'my last message'", 4, move || {
            Box::pin(async move {
                let response = get_context("my last message", port, CannedUser::Dev1)
                    .await?
                    .to_success()
                    .context("to_success")?;

                println!(
                    "RESPONSE:\n{}",
                    serde_json::to_string_pretty(&response).unwrap()
                );

                Ok(vec![
                    response.data.contains("Awesome!!"),
                    response.plan.executed_steps.len() == 2,
                    query_planner_step_matches_range(
                        response.plan.executed_steps[0]
                            .as_query()
                            .context("to_query")?,
                        10..50,
                        serde_json::from_value(json!({
                            "Slack": ["Messages"]
                        }))
                        .unwrap(),
                        Confidence::Certain,
                    ),
                    return_planner_step_matches(
                        response.plan.executed_steps[1]
                            .as_return()
                            .context("to_return")?,
                        Some(
                            serde_json::from_value(json!({
                                "0": { "Slack": ["Messages"] }
                            }))
                            .unwrap(),
                        ),
                        None,
                        Confidence::Optimistic,
                        None,
                    ),
                ])
            })
        })
        .await;

        // Simple query to test thread retrieval
        //
        // This test verifies:
        // - Simple two step plan
        test.start_test_sequence("'the last thread in random'", 4, move || {
            Box::pin(async move {
                let response = get_context("the last thread in random", port, CannedUser::Dev1)
                    .await?
                    .to_success()
                    .context("to_success")?;

                println!(
                    "RESPONSE:\n{}",
                    serde_json::to_string_pretty(&response).unwrap()
                );

                Ok(vec![
                    response.data.contains("could be us") && response.data.contains("will be us"),
                    response.plan.executed_steps.len() == 2,
                    query_planner_step_matches_range(
                        response.plan.executed_steps[0]
                            .as_query()
                            .context("to_query")?,
                        10..50,
                        serde_json::from_value(json!({
                            "Slack": ["Messages"]
                        }))
                        .unwrap(),
                        Confidence::Certain,
                    ),
                    return_planner_step_matches(
                        response.plan.executed_steps[1]
                            .as_return()
                            .context("to_return")?,
                        Some(
                            serde_json::from_value(json!({
                                "0": { "Slack": ["Messages"] }
                            }))
                            .unwrap(),
                        ),
                        None,
                        Confidence::Optimistic,
                        None,
                    ),
                ])
            })
        })
        .await;

        // Simple two step query to get a response to Ryan
        //
        // This is actually a pretty complicated test and there are a number of ways to
        // get the right answer
        test.start_test_sequence(
            "'what was the response to Ryan about social links for sundry'",
            1,
            move || {
                Box::pin(async move {
                    let response = get_context(
                        "what was the response to Ryan's question about social links for sundry",
                        port,
                        CannedUser::Dev1,
                    )
                    .await?
                    .to_success()
                    .context("to_success")?;

                    println!(
                        "RESPONSE:\n{}",
                        serde_json::to_string_pretty(&response).unwrap()
                    );

                    Ok(vec![response.data.contains("Not for bluesky")])
                })
            },
        )
        .await;

        test
    }

    async fn score_github(port: u16) -> TestResult {
        let mut test = TestResult::new("github end->end");

        // Simple query with no results
        //
        // This test verifies:
        // - No unecessary query rewriting
        // - Simple two step plan
        // - No error when returning empty data
        test.start_test_sequence("'my open GitHub issues' with empty results", 4, move || {
            Box::pin(async move {
                let response = get_context("my open GitHub issues", port, CannedUser::Dev1)
                    .await?
                    .to_success()
                    .context("to_success")?;

                println!(
                    "RESPONSE:\n{}",
                    serde_json::to_string_pretty(&response).unwrap()
                );

                Ok(vec![
                    response.data.len() <= 25,
                    response.plan.executed_steps.len() == 2,
                    query_planner_step_matches(
                        response.plan.executed_steps[0]
                            .as_query()
                            .context("to_query")?,
                        "my open GitHub issues",
                        serde_json::from_value(json!({
                            "GitHub": ["Issues"]
                        }))
                        .unwrap(),
                        Confidence::Certain,
                    ),
                    return_planner_step_matches(
                        response.plan.executed_steps[1]
                            .as_return()
                            .context("to_return")?,
                        Some(
                            serde_json::from_value(json!({
                                "0": { "GitHub": ["Issues"] }
                            }))
                            .unwrap(),
                        ),
                        None,
                        Confidence::Optimistic,
                        Some(Some(10..200)),
                    ),
                ])
            })
        })
        .await;

        // Simple query with 1 result
        //
        // This test verifies:
        // - No unecessary query rewriting
        // - Simple two step plan
        // - No unecessary data transformations
        test.start_test_sequence(
            "'open github issues assigned to me' with 1 result",
            4,
            move || {
                Box::pin(async move {
                    let response =
                        get_context("open GitHub issues assigned to me", port, CannedUser::Dev1)
                            .await?
                            .to_success()
                            .context("to_success")?;

                    println!(
                        "RESPONSE:\n{}",
                        serde_json::to_string_pretty(&response).unwrap()
                    );

                    Ok(vec![
                        response.data.contains("provide some overviews of models"), // The title of the issue
                        response.plan.executed_steps.len() == 2,
                        query_planner_step_matches(
                            response.plan.executed_steps[0]
                                .as_query()
                                .context("to_query")?,
                            "open GitHub issues assigned to me",
                            serde_json::from_value(json!({
                                "GitHub": ["Issues"]
                            }))
                            .unwrap(),
                            Confidence::Certain,
                        ),
                        return_planner_step_matches(
                            response.plan.executed_steps[1]
                                .as_return()
                                .context("to_return")?,
                            Some(
                                serde_json::from_value(json!({
                                    "0": { "GitHub": ["Issues"] }
                                }))
                                .unwrap(),
                            ),
                            None,
                            Confidence::Optimistic,
                            None,
                        ),
                    ])
                })
            },
        )
        .await;

        // Simple query with 1 result
        //
        // This test verifies:
        // - No unecessary query rewriting
        // - Simple two step plan
        // - Correct use of time when querying
        // - No unecessary data transformations
        test.start_test_sequence(
            "'issues I created in August of 2024' with 1 result",
            4,
            move || {
                Box::pin(async move {
                    let response =
                        get_context("issues I created in August of 2024", port, CannedUser::Dev1)
                            .await?
                            .to_success()
                            .context("to_success")?;

                    Ok(vec![
                        response.data.contains("Pull Ollama Models"), // The title of the issue
                        response.plan.executed_steps.len() == 2,
                        query_planner_step_matches(
                            response.plan.executed_steps[0]
                                .as_query()
                                .context("to_query")?,
                            "issues I created in August of 2024",
                            serde_json::from_value(json!({
                                "GitHub": ["Issues"]
                            }))
                            .unwrap(),
                            Confidence::Certain,
                        ),
                        return_planner_step_matches(
                            response.plan.executed_steps[1]
                                .as_return()
                                .context("to_return")?,
                            Some(
                                serde_json::from_value(json!({
                                    "0": { "GitHub": ["Issues"] }
                                }))
                                .unwrap(),
                            ),
                            None,
                            Confidence::Optimistic,
                            None,
                        ),
                    ])
                })
            },
        )
        .await;

        // Simple query with 2 results
        //
        // This test verifies:
        // - No unecessary query rewriting
        // - Simple two step plan
        // - Correct use of searching through comments
        // - No unecessary data transformations
        test.start_test_sequence(
            "'pull requests I commented on' with 1 result",
            5,
            move || {
                Box::pin(async move {
                    let response =
                        get_context("pull requests I commented on", port, CannedUser::Dev2)
                            .await?
                            .to_success()
                            .context("to_success")?;

                    println!("{}", serde_json::to_string_pretty(&response).unwrap());

                    Ok(vec![
                        response.data.contains("Simplified nix flake"), // Pull request 1
                        response.data.contains("Release/v0.7.0"),       // Pull request 2
                        response.plan.executed_steps.len() == 2,
                        query_planner_step_matches(
                            response.plan.executed_steps[0]
                                .as_query()
                                .context("to_query")?,
                            "pull requests I commented on",
                            serde_json::from_value(json!({
                                "GitHub": ["PullRequests"]
                            }))
                            .unwrap(),
                            Confidence::Certain,
                        ),
                        return_planner_step_matches(
                            response.plan.executed_steps[1]
                                .as_return()
                                .context("to_return")?,
                            Some(
                                serde_json::from_value(json!({
                                    "0": { "GitHub": ["PullRequests"] }
                                }))
                                .unwrap(),
                            ),
                            None,
                            Confidence::Optimistic,
                            None,
                        ),
                    ])
                })
            },
        )
        .await;

        // Two stage query with 2 results
        //
        // This test verifies:
        // - Query rewriting when appropriate
        // - Three step plan
        // - No unecessary data transformations
        // - Assumptions are shared correctly
        test.start_test_sequence(
            "'GitHub pull requests referencing the open GitHub issue about adding nix flakes' with many results",
            6,
            move || {
                Box::pin(async move {
                    let response = get_context(
                        "GitHub pull requests referencing the open GitHub issue about adding nix flakes",
                        port,
                        CannedUser::Dev1,
                    )
                    .await?
                    .to_success()
                    .context("to_success")?;

                    Ok(vec![
                        response.data.contains("Flake"), // Pull request 1
                        response.data.contains("Simplified nix flake"), // Pull request 2
                        response.plan.executed_steps.len() == 3,
                        query_planner_step_matches_range(
                            response.plan.executed_steps[0]
                                .as_query()
                                .context("to_query")?,
                            10..50,
                            serde_json::from_value(json!({
                                "GitHub": ["Issues"]
                            })).unwrap(),
                            Confidence::Optimistic,
                        ),
                        query_planner_step_matches_range(
                            response.plan.executed_steps[1]
                                .as_query()
                                .context("to_query")?,
                            10..100,
                            serde_json::from_value(json!({
                                "GitHub": ["PullRequests"]
                            })).unwrap(),
                            Confidence::Optimistic,
                        ),
                        return_planner_step_matches(
                            response.plan.executed_steps[2]
                                .as_return()
                                .context("to_return")?,
                            serde_json::from_value(json!({
                                "1": {
                                    "GitHub": ["PullRequests"]
                                }
                            })).unwrap(),
                            None,
                            Confidence::Optimistic,
                            Some(Some(10..200)),
                        ),
                    ])
                })
            },
        )
        .await;

        // Should add a transform test

        test
    }

    async fn score_github_slack(port: u16) -> TestResult {
        let mut test = TestResult::new("github x slack end->end");

        // Multi-step query first checking GitHub and then Slack returning empty results
        //
        // This test verifies:
        // - Correctly uses summarize
        // - Can handle multi-step query
        // - Can correctly convey assumptions
        test.start_test_sequence(
            "'has anyone in slack mentioned my last github issue at all'",
            5,
            move || {
                Box::pin(async move {
                    let response = get_context(
                        "has anyone in slack mentioned my last github issue at all",
                        port,
                        CannedUser::Dev1,
                    )
                    .await?
                    .to_success()
                    .context("to_success")?;

                    println!(
                        "RESPONSE:\n{}",
                        serde_json::to_string_pretty(&response).unwrap()
                    );

                    Ok(vec![
                        response.data.len() < 100, // We should get nothing
                        response.plan.executed_steps.len() == 3,
                        query_planner_step_matches_range(
                            response.plan.executed_steps[0]
                                .as_query()
                                .context("to_query")?,
                            10..100,
                            serde_json::from_value(json!({
                                "GitHub": ["Issues"]
                            }))
                            .unwrap(),
                            Confidence::Optimistic,
                        ),
                        query_planner_step_matches_range(
                            response.plan.executed_steps[1]
                                .as_query()
                                .context("to_query")?,
                            25..200,
                            serde_json::from_value(json!({
                                "Slack": ["Messages"]
                            }))
                            .unwrap(),
                            Confidence::Optimistic,
                        ),
                        return_planner_step_matches(
                            response.plan.executed_steps[2]
                                .as_return()
                                .context("to_return")?,
                            Some(
                                serde_json::from_value(json!({
                                    "1": { "Slack": ["Messages"] }
                                }))
                                .unwrap(),
                            ),
                            None,
                            Confidence::Optimistic,
                            Some(Some(10..200)),
                        ),
                    ])
                })
            },
        )
        .await;

        test
    }
}
