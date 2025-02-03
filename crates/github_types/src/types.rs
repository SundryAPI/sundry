use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvents {
    Created(CreatedEvent),
    // Discussion,
    // DiscussionComment,
    Issues(IssueEvent),
    IssueComment,
    // Project,
    // ProjectsV2Item,
    // ProjectCard,
    // ProjectColumn,
    // Public,
    // PullRequest,
    // PullRequestReview,
    // PullRequestReviewComment,
    // PullRequestReviewThread,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Read,
    #[serde(untagged)]
    Other(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Permissions {
    pub discussions: Option<Permission>,
    pub issues: Option<Permission>,
    pub metadata: Option<Permission>,
    pub pull_requests: Option<Permission>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Discussion,
    DiscussionComment,
    Issues,
    IssueComment,
    PullRequest,
    PullRequestReviewComment,
    PullRequestReviewThread,
    SubIssues,
    #[serde(untagged)]
    Other(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct CreatedEvent {
    pub installation: FullInstallation,
    pub repositories: Vec<Repository>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct FullInstallation {
    pub id: i64,
    pub account: User,
    pub permissions: Permissions,
    pub events: Vec<Event>,
}

impl FullInstallation {
    pub fn permisions_meets_requirements(&self) -> bool {
        self.permissions
            .discussions
            .as_ref()
            .and_then(|p| {
                if matches!(p, Permission::Read) {
                    Some(())
                } else {
                    None
                }
            })
            .and(self.permissions.issues.as_ref())
            .and_then(|p| {
                if matches!(p, Permission::Read) {
                    Some(())
                } else {
                    None
                }
            })
            .and(self.permissions.metadata.as_ref())
            .and_then(|p| {
                if matches!(p, Permission::Read) {
                    Some(())
                } else {
                    None
                }
            })
            .and(self.permissions.pull_requests.as_ref())
            .and_then(|p| {
                if matches!(p, Permission::Read) {
                    Some(())
                } else {
                    None
                }
            })
            .is_some()
    }

    pub fn events_meet_requirements(&self) -> bool {
        self.events
            == vec![
                Event::Discussion,
                Event::DiscussionComment,
                Event::Issues,
                Event::IssueComment,
                Event::PullRequest,
                Event::PullRequestReviewComment,
                Event::PullRequestReviewThread,
                Event::SubIssues,
            ]
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Installation {
    pub id: u64,
    pub node_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub private: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct IssueEvent {
    pub action: IssueAction,
    pub issue: Issue,
    pub repository: Repository,
    pub installation: Installation,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueState {
    Open,
    Closed,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Issue {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub url: Url,
    pub comments_url: Url,
    pub user: User,
    pub state: IssueState,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum IssueAction {
    Opened,
    Closed,
    Edited,
    Deleted,
    #[serde(untagged)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const ISSUE_OPENED: &str = r##"
    {
       "action":"opened",
       "issue":{
          "url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/6",
          "repository_url":"https://api.github.com/repos/SilasMarvin/test-github-app",
          "labels_url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/6/labels{/name}",
          "comments_url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/6/comments",
          "events_url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/6/events",
          "html_url":"https://github.com/SilasMarvin/test-github-app/issues/6",
          "id":2553184805,
          "node_id":"I_kwDOM3u-j86YLoIl",
          "number":6,
          "title":"Test 6",
          "user":{
             "login":"SilasMarvin",
             "id":19626586,
             "node_id":"MDQ6VXNlcjE5NjI2NTg2",
             "avatar_url":"https://avatars.githubusercontent.com/u/19626586?v=4",
             "gravatar_id":"",
             "url":"https://api.github.com/users/SilasMarvin",
             "html_url":"https://github.com/SilasMarvin",
             "followers_url":"https://api.github.com/users/SilasMarvin/followers",
             "following_url":"https://api.github.com/users/SilasMarvin/following{/other_user}",
             "gists_url":"https://api.github.com/users/SilasMarvin/gists{/gist_id}",
             "starred_url":"https://api.github.com/users/SilasMarvin/starred{/owner}{/repo}",
             "subscriptions_url":"https://api.github.com/users/SilasMarvin/subscriptions",
             "organizations_url":"https://api.github.com/users/SilasMarvin/orgs",
             "repos_url":"https://api.github.com/users/SilasMarvin/repos",
             "events_url":"https://api.github.com/users/SilasMarvin/events{/privacy}",
             "received_events_url":"https://api.github.com/users/SilasMarvin/received_events",
             "type":"User",
             "site_admin":false
          },
          "labels":[

          ],
          "state":"open",
          "locked":false,
          "assignee":null,
          "assignees":[

          ],
          "milestone":null,
          "comments":0,
          "created_at":"2024-09-27T15:14:41Z",
          "updated_at":"2024-09-27T15:14:41Z",
          "closed_at":null,
          "author_association":"OWNER",
          "active_lock_reason":null,
          "body":null,
          "reactions":{
             "url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/6/reactions",
             "total_count":0,
             "+1":0,
             "-1":0,
             "laugh":0,
             "hooray":0,
             "confused":0,
             "heart":0,
             "rocket":0,
             "eyes":0
          },
          "timeline_url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/6/timeline",
          "performed_via_github_app":null,
          "state_reason":null
       },
       "repository":{
          "id":863747727,
          "node_id":"R_kgDOM3u-jw",
          "name":"test-github-app",
          "full_name":"SilasMarvin/test-github-app",
          "private":true,
          "owner":{
             "login":"SilasMarvin",
             "id":19626586,
             "node_id":"MDQ6VXNlcjE5NjI2NTg2",
             "avatar_url":"https://avatars.githubusercontent.com/u/19626586?v=4",
             "gravatar_id":"",
             "url":"https://api.github.com/users/SilasMarvin",
             "html_url":"https://github.com/SilasMarvin",
             "followers_url":"https://api.github.com/users/SilasMarvin/followers",
             "following_url":"https://api.github.com/users/SilasMarvin/following{/other_user}",
             "gists_url":"https://api.github.com/users/SilasMarvin/gists{/gist_id}",
             "starred_url":"https://api.github.com/users/SilasMarvin/starred{/owner}{/repo}",
             "subscriptions_url":"https://api.github.com/users/SilasMarvin/subscriptions",
             "organizations_url":"https://api.github.com/users/SilasMarvin/orgs",
             "repos_url":"https://api.github.com/users/SilasMarvin/repos",
             "events_url":"https://api.github.com/users/SilasMarvin/events{/privacy}",
             "received_events_url":"https://api.github.com/users/SilasMarvin/received_events",
             "type":"User",
             "site_admin":false
          },
          "html_url":"https://github.com/SilasMarvin/test-github-app",
          "description":null,
          "fork":false,
          "url":"https://api.github.com/repos/SilasMarvin/test-github-app",
          "forks_url":"https://api.github.com/repos/SilasMarvin/test-github-app/forks",
          "keys_url":"https://api.github.com/repos/SilasMarvin/test-github-app/keys{/key_id}",
          "collaborators_url":"https://api.github.com/repos/SilasMarvin/test-github-app/collaborators{/collaborator}",
          "teams_url":"https://api.github.com/repos/SilasMarvin/test-github-app/teams",
          "hooks_url":"https://api.github.com/repos/SilasMarvin/test-github-app/hooks",
          "issue_events_url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/events{/number}",
          "events_url":"https://api.github.com/repos/SilasMarvin/test-github-app/events",
          "assignees_url":"https://api.github.com/repos/SilasMarvin/test-github-app/assignees{/user}",
          "branches_url":"https://api.github.com/repos/SilasMarvin/test-github-app/branches{/branch}",
          "tags_url":"https://api.github.com/repos/SilasMarvin/test-github-app/tags",
          "blobs_url":"https://api.github.com/repos/SilasMarvin/test-github-app/git/blobs{/sha}",
          "git_tags_url":"https://api.github.com/repos/SilasMarvin/test-github-app/git/tags{/sha}",
          "git_refs_url":"https://api.github.com/repos/SilasMarvin/test-github-app/git/refs{/sha}",
          "trees_url":"https://api.github.com/repos/SilasMarvin/test-github-app/git/trees{/sha}",
          "statuses_url":"https://api.github.com/repos/SilasMarvin/test-github-app/statuses/{sha}",
          "languages_url":"https://api.github.com/repos/SilasMarvin/test-github-app/languages",
          "stargazers_url":"https://api.github.com/repos/SilasMarvin/test-github-app/stargazers",
          "contributors_url":"https://api.github.com/repos/SilasMarvin/test-github-app/contributors",
          "subscribers_url":"https://api.github.com/repos/SilasMarvin/test-github-app/subscribers",
          "subscription_url":"https://api.github.com/repos/SilasMarvin/test-github-app/subscription",
          "commits_url":"https://api.github.com/repos/SilasMarvin/test-github-app/commits{/sha}",
          "git_commits_url":"https://api.github.com/repos/SilasMarvin/test-github-app/git/commits{/sha}",
          "comments_url":"https://api.github.com/repos/SilasMarvin/test-github-app/comments{/number}",
          "issue_comment_url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues/comments{/number}",
          "contents_url":"https://api.github.com/repos/SilasMarvin/test-github-app/contents/{+path}",
          "compare_url":"https://api.github.com/repos/SilasMarvin/test-github-app/compare/{base}...{head}",
          "merges_url":"https://api.github.com/repos/SilasMarvin/test-github-app/merges",
          "archive_url":"https://api.github.com/repos/SilasMarvin/test-github-app/{archive_format}{/ref}",
          "downloads_url":"https://api.github.com/repos/SilasMarvin/test-github-app/downloads",
          "issues_url":"https://api.github.com/repos/SilasMarvin/test-github-app/issues{/number}",
          "pulls_url":"https://api.github.com/repos/SilasMarvin/test-github-app/pulls{/number}",
          "milestones_url":"https://api.github.com/repos/SilasMarvin/test-github-app/milestones{/number}",
          "notifications_url":"https://api.github.com/repos/SilasMarvin/test-github-app/notifications{?since,all,participating}",
          "labels_url":"https://api.github.com/repos/SilasMarvin/test-github-app/labels{/name}",
          "releases_url":"https://api.github.com/repos/SilasMarvin/test-github-app/releases{/id}",
          "deployments_url":"https://api.github.com/repos/SilasMarvin/test-github-app/deployments",
          "created_at":"2024-09-26T20:54:18Z",
          "updated_at":"2024-09-26T21:18:14Z",
          "pushed_at":"2024-09-26T21:18:11Z",
          "git_url":"git://github.com/SilasMarvin/test-github-app.git",
          "ssh_url":"git@github.com:SilasMarvin/test-github-app.git",
          "clone_url":"https://github.com/SilasMarvin/test-github-app.git",
          "svn_url":"https://github.com/SilasMarvin/test-github-app",
          "homepage":null,
          "size":6,
          "stargazers_count":0,
          "watchers_count":0,
          "language":"JavaScript",
          "has_issues":true,
          "has_projects":true,
          "has_downloads":true,
          "has_wiki":true,
          "has_pages":false,
          "has_discussions":false,
          "forks_count":0,
          "mirror_url":null,
          "archived":false,
          "disabled":false,
          "open_issues_count":3,
          "license":null,
          "allow_forking":true,
          "is_template":false,
          "web_commit_signoff_required":false,
          "topics":[

          ],
          "visibility":"private",
          "forks":0,
          "open_issues":3,
          "watchers":0,
          "default_branch":"main"
       },
       "sender":{
          "login":"SilasMarvin",
          "id":19626586,
          "node_id":"MDQ6VXNlcjE5NjI2NTg2",
          "avatar_url":"https://avatars.githubusercontent.com/u/19626586?v=4",
          "gravatar_id":"",
          "url":"https://api.github.com/users/SilasMarvin",
          "html_url":"https://github.com/SilasMarvin",
          "followers_url":"https://api.github.com/users/SilasMarvin/followers",
          "following_url":"https://api.github.com/users/SilasMarvin/following{/other_user}",
          "gists_url":"https://api.github.com/users/SilasMarvin/gists{/gist_id}",
          "starred_url":"https://api.github.com/users/SilasMarvin/starred{/owner}{/repo}",
          "subscriptions_url":"https://api.github.com/users/SilasMarvin/subscriptions",
          "organizations_url":"https://api.github.com/users/SilasMarvin/orgs",
          "repos_url":"https://api.github.com/users/SilasMarvin/repos",
          "events_url":"https://api.github.com/users/SilasMarvin/events{/privacy}",
          "received_events_url":"https://api.github.com/users/SilasMarvin/received_events",
          "type":"User",
          "site_admin":false
       },
       "installation":{
          "id":55350473,
          "node_id":"MDIzOkludGVncmF0aW9uSW5zdGFsbGF0aW9uNTUzNTA0NzM="
       }
    }
    "##;

    const CREATED: &str = r##"
        {
           "action":"created",
           "installation":{
              "id":55394148,
              "client_id":"Iv23liO5LMyFJHA39pE1",
              "account":{
                 "login":"SilasMarvin",
                 "id":19626586,
                 "node_id":"MDQ6VXNlcjE5NjI2NTg2",
                 "avatar_url":"https://avatars.githubusercontent.com/u/19626586?v=4",
                 "gravatar_id":"",
                 "url":"https://api.github.com/users/SilasMarvin",
                 "html_url":"https://github.com/SilasMarvin",
                 "followers_url":"https://api.github.com/users/SilasMarvin/followers",
                 "following_url":"https://api.github.com/users/SilasMarvin/following{/other_user}",
                 "gists_url":"https://api.github.com/users/SilasMarvin/gists{/gist_id}",
                 "starred_url":"https://api.github.com/users/SilasMarvin/starred{/owner}{/repo}",
                 "subscriptions_url":"https://api.github.com/users/SilasMarvin/subscriptions",
                 "organizations_url":"https://api.github.com/users/SilasMarvin/orgs",
                 "repos_url":"https://api.github.com/users/SilasMarvin/repos",
                 "events_url":"https://api.github.com/users/SilasMarvin/events{/privacy}",
                 "received_events_url":"https://api.github.com/users/SilasMarvin/received_events",
                 "type":"User",
                 "site_admin":false
              },
              "repository_selection":"selected",
              "access_tokens_url":"https://api.github.com/app/installations/55394148/access_tokens",
              "repositories_url":"https://api.github.com/installation/repositories",
              "html_url":"https://github.com/settings/installations/55394148",
              "app_id":1009337,
              "app_slug":"silas-test-webhook",
              "target_id":19626586,
              "target_type":"User",
              "permissions":{
                 "discussions":"read",
                 "issues":"read",
                 "metadata":"read",
                 "pull_requests":"read"
              },
              "events":[
                 "discussion",
                 "discussion_comment",
                 "issues",
                 "issue_comment",
                 "pull_request",
                 "pull_request_review_comment",
                 "pull_request_review_thread",
                 "sub_issues"
              ],
              "created_at":"2024-09-28T16:45:27.000-07:00",
              "updated_at":"2024-09-28T16:45:27.000-07:00",
              "single_file_name":null,
              "has_multiple_single_files":false,
              "single_file_paths":[

              ],
              "suspended_by":null,
              "suspended_at":null
           },
           "repositories":[
              {
                 "id":863747727,
                 "node_id":"R_kgDOM3u-jw",
                 "name":"test-github-app",
                 "full_name":"SilasMarvin/test-github-app",
                 "private":true
              }
           ],
           "requester":null,
           "sender":{
              "login":"SilasMarvin",
              "id":19626586,
              "node_id":"MDQ6VXNlcjE5NjI2NTg2",
              "avatar_url":"https://avatars.githubusercontent.com/u/19626586?v=4",
              "gravatar_id":"",
              "url":"https://api.github.com/users/SilasMarvin",
              "html_url":"https://github.com/SilasMarvin",
              "followers_url":"https://api.github.com/users/SilasMarvin/followers",
              "following_url":"https://api.github.com/users/SilasMarvin/following{/other_user}",
              "gists_url":"https://api.github.com/users/SilasMarvin/gists{/gist_id}",
              "starred_url":"https://api.github.com/users/SilasMarvin/starred{/owner}{/repo}",
              "subscriptions_url":"https://api.github.com/users/SilasMarvin/subscriptions",
              "organizations_url":"https://api.github.com/users/SilasMarvin/orgs",
              "repos_url":"https://api.github.com/users/SilasMarvin/repos",
              "events_url":"https://api.github.com/users/SilasMarvin/events{/privacy}",
              "received_events_url":"https://api.github.com/users/SilasMarvin/received_events",
              "type":"User",
              "site_admin":false
           }
        }
    "##;

    #[test]
    fn deserialize_issue() {
        let issue_event: WebhookEvents = serde_json::from_str(ISSUE_OPENED).unwrap();
        assert!(matches!(issue_event, WebhookEvents::Issues(_)));
    }

    #[test]
    fn deserialize_created() {
        let created_event: WebhookEvents = serde_json::from_str(CREATED).unwrap();
        assert!(matches!(created_event, WebhookEvents::Created(_)));
    }
}
