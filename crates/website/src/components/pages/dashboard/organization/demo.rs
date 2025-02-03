use futures::channel::mpsc;
use futures::StreamExt;
use leptos::html::Div;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{CloseEvent, ErrorEvent, HtmlDivElement, MessageEvent, WebSocket};

use models::{organization::Organization, source::Sources};

#[cfg(feature = "ssr")]
use crate::AppState;
use crate::{
    components::{
        atoms::loading::{LoadingWheel, LoadingWheelSize},
        molecules::modal::Modal,
    },
    PUBLIC_DEV_MODE,
};

#[cfg(feature = "ssr")]
use axum::{
    extract::{ws, Path, State},
    response::Response,
};

#[cfg(feature = "ssr")]
use http::{HeaderMap, StatusCode};

#[cfg(feature = "ssr")]
use async_openai::{
    error::OpenAIError,
    types::{
        ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
        ChatCompletionToolArgs, ChatCompletionToolType, CreateChatCompletionRequestArgs,
        FinishReason, FunctionCall, FunctionObjectArgs,
    },
    Client,
};

#[cfg(feature = "ssr")]
use futures::stream::BoxStream;

#[cfg(feature = "ssr")]
const SYSTEM_PROMPT: &str = r##"
You are a helpful and friendly chatbot.

You have access to the get_context function which can query information about the user's: {SOURCES}

When using get_context, phrase queries naturally as if you were the user, for example:
- "my most recent github issue"
- "the last pull request I created"
- "any github issues assigned to me"
- "show me open pull requests from last week"
- "find github discussions about the authentication bug"
- "what's the status of issue #123"
- "who commented on my latest PR"

DO NOT use technical or API-style queries like:
- "query=issues.latest"
- "GET /issues?state=open"
- "filter:issue.author"

DO NOT provide subjective queries like:
- "easy github issues"
- "fun pull requests"
Instead query for the items directly and provide your own subjective filtering over the data.

get_context returns data, a confidence level, and a user_message:
- confidence is one of "certain", "optimistic", "tentative", and "doubtful" and represents the confidence the returned data matches the users query
- the user_message contains important assumptions made while gathering the context
- YOU MUST ALWAYS COMMUNICATE THE USER_MESSAGE to help users understand how their request was interpreted
- Share these assumptions naturally, like "I looked at PRs from the last 7 days" or "I focused on issues tagged as bugs"
- combinations of low confidence and assumptions typically means more specific queries will return better data

Best practices:
- Use natural, conversational language in queries
- Keep queries specific and focused
- Mirror the user's language and intent
- Analyze and summarize the returned context in a helpful way
- CLEARLY COMMUNICATE any assumptions from the user_message conversationally as part of your response
- Offer to ask for more specific data where you feel appropriate
- If you get an error, explain it simply and suggest alternatives
- CRITICAL: Always ask for the exact data you want. If you want commentors on a pull request ask for commentors on a pull request not just the pull request

Remember: You're having a conversation, so keep responses natural and helpful while ensuring users understand how their requests were interpreted.
"##;

#[cfg(feature = "ssr")]
async fn get_context(
    query: &str,
    user_id: i64,
    organization_id: i64,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let output = client
        .post(
            std::env::var("SUNDRY_DEMO_API_ENDPOINT")
                .expect("missing SUNDRY_DEMO_API_ENDPOINT env variable"),
        )
        .header(
            "X-API-Key",
            std::env::var("SUNDRY_DEMO_API_APPLICATION_KEY")
                .expect("missing SUNDRY_DEMO_API_APPLICATION_KEY env variable"),
        )
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "query": query,
            "user": {
                "id": user_id,
                "organization_id": organization_id
            }
        }))
        .send()
        .await?
        .text()
        .await?;

    Ok(output)
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct GetContextToolArguments {
    query: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum WebSocketResponse {
    ChatError(String),
    NoAPIKeys,
    Sources(Sources),
    Success(ChatMessage),
}

#[cfg(feature = "ssr")]
impl From<WebSocketResponse> for ws::Message {
    fn from(val: WebSocketResponse) -> Self {
        ws::Message::Text(serde_json::to_string(&val).unwrap())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum MessageContent {
    User(UserMessage),
    Assistant(AssistantMessage),
    ToolCall(ToolCallMessage),
    ToolResponse(ToolResponseMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChatMessage {
    id: u32,
    content: MessageContent,
}

impl ChatMessage {
    fn new_user_message(id: u32, content: &str) -> Self {
        ChatMessage {
            id,
            content: MessageContent::User(UserMessage {
                message: content.to_string(),
            }),
        }
    }

    fn merge(&mut self, other: ChatMessage) {
        // We only merge assistant messages that come from the server
        // This won't be used for other messages
        if let (
            ChatMessage {
                content: MessageContent::Assistant(ref mut existing),
                ..
            },
            ChatMessage {
                content: MessageContent::Assistant(new),
                ..
            },
        ) = (self, other)
        {
            existing.message.push_str(&new.message);
        };
    }

    // This cannot just be the id of the message as we stream the rsponse from the
    // assistant updating the content of the message with the same id. This means
    // leptos won't re-render it if we use the id
    fn key(&self) -> String {
        match &self.content {
            MessageContent::User(user) => {
                format!("{}-{}", self.id, user.message)
            }
            MessageContent::Assistant(assistant) => {
                format!("{}-{}", self.id, assistant.message)
            }
            MessageContent::ToolCall(tool_call) => format!("{}-{}", self.id, tool_call.arguments),
            MessageContent::ToolResponse(tool_response) => {
                format!("{}-{}", self.id, tool_response.content)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserMessage {
    message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AssistantMessage {
    message: String,
    is_done: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ToolCallMessage {
    id: String,
    arguments: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ToolResponseMessage {
    id: String,
    content: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
enum StreamResponseMessage {
    ToolCall(ToolCallMessage),
    Assistant(AssistantMessage),
}

#[cfg(feature = "ssr")]
fn chat_messages_to_openai_messages(
    messages: &[ChatMessage],
) -> Result<Vec<ChatCompletionRequestMessage>, OpenAIError> {
    let mut new_messages = vec![];

    for message in messages {
        let new_message = match &message.content {
            MessageContent::User(chat_message) => ChatCompletionRequestUserMessageArgs::default()
                .content(chat_message.message.to_string())
                .build()?
                .into(),
            MessageContent::Assistant(assistant_message) => {
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content(assistant_message.message.to_string())
                    .build()?
                    .into()
            }
            MessageContent::ToolCall(tool_call) => {
                ChatCompletionRequestAssistantMessageArgs::default()
                    .tool_calls(vec![ChatCompletionMessageToolCall {
                        id: tool_call.id.clone(),
                        r#type: ChatCompletionToolType::Function,
                        function: FunctionCall {
                            name: "get_context".to_string(),
                            arguments: tool_call.arguments.clone(),
                        },
                    }])
                    .build()?
                    .into()
            }
            MessageContent::ToolResponse(tool_response) => {
                ChatCompletionRequestToolMessageArgs::default()
                    .content(tool_response.content.clone())
                    .tool_call_id(tool_response.id.clone())
                    .build()?
                    .into()
            }
        };

        match new_messages.last() {
            Some(last_message) => {
                let should_replace = matches!(
                    (&message.content, last_message),
                    (
                        MessageContent::User(_),
                        ChatCompletionRequestMessage::User(_)
                    ) | (
                        MessageContent::Assistant(_),
                        ChatCompletionRequestMessage::Assistant(_)
                    ) | (
                        MessageContent::ToolCall(_),
                        ChatCompletionRequestMessage::Assistant(_)
                    ) | (
                        MessageContent::ToolResponse(_),
                        ChatCompletionRequestMessage::Tool(_)
                    )
                );

                if should_replace {
                    *new_messages.last_mut().unwrap() = new_message;
                } else {
                    new_messages.push(new_message);
                }
            }
            None => new_messages.push(new_message),
        }
    }

    Ok(new_messages)
}

#[cfg(feature = "ssr")]
async fn create_chat_stream<'a, 'b>(
    messages: &'a [ChatMessage],
    sources: &'a Sources,
) -> Result<BoxStream<'a, Result<StreamResponseMessage, OpenAIError>>, OpenAIError> {
    let mut conversation_messages = chat_messages_to_openai_messages(messages)?;
    let source_specific_prompt = SYSTEM_PROMPT.replace(
        "{SOURCES}",
        &serde_json::to_string_pretty(&sources).unwrap(),
    );
    let mut messages = vec![ChatCompletionRequestSystemMessageArgs::default()
        .content(source_specific_prompt)
        .build()?
        .into()];
    messages.append(&mut conversation_messages);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(4096u32)
        .model("gpt-4o")
        .messages(messages)
        .tools(vec![ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name("get_context")
                    .description("Query the GitHub context API for information about issues.")
                    .parameters(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "The GitHub-related query to search for in plain text",
                            }
                        },
                        "required": ["query"],
                        "additionalProperties": false,
                    }))
                    .build()?,
            )
            .build()?])
        .build()?;

    let client = Client::new();
    let stream = client.chat().create_stream(request).await?;

    // A couple things to note:
    // - We assume there is only one choice returned. I.E. n = 1
    // - We assume the llm only calls get_context and only ever once per rsponse
    let stream = futures::stream::try_unfold(stream, |mut stream| async move {
        let mut current_tool_call = None;

        while let Some(response) = stream.next().await {
            let response = response?;
            if let Some(choice) = response.choices.first() {
                // Handle tool call response
                if let Some(tool_call) = choice
                    .delta
                    .tool_calls
                    .as_ref()
                    .and_then(|calls| calls.first())
                {
                    // We know the name and the type is function we don't need to verify that here
                    // Note that the function to be called is sent in the first message the
                    // arguments are streamed
                    if current_tool_call.is_none() {
                        current_tool_call = Some(ToolCallMessage {
                            id: tool_call.id.as_deref().unwrap_or_default().to_string(),
                            arguments: tool_call
                                .function
                                .as_ref()
                                .and_then(|f| f.arguments.clone())
                                .unwrap_or_default(),
                        });
                    } else if let Some(arguments) = tool_call
                        .function
                        .as_ref()
                        .and_then(|f| f.arguments.as_ref())
                    {
                        current_tool_call
                            .as_mut()
                            .unwrap()
                            .arguments
                            .push_str(arguments);
                    }
                }

                // Handle normal content response
                if let Some(content) = &choice.delta.content {
                    return Ok(Some((
                        StreamResponseMessage::Assistant(AssistantMessage {
                            message: content.to_string(),
                            is_done: choice.finish_reason.is_some(),
                        }),
                        stream,
                    )));
                }

                // Handle finish
                if let Some(finish_reason) = &choice.finish_reason {
                    match (finish_reason, current_tool_call) {
                        (FinishReason::ToolCalls, Some(tool_call)) => {
                            return Ok(Some((StreamResponseMessage::ToolCall(tool_call), stream)));
                        }
                        _ => {
                            return Ok(Some((
                                StreamResponseMessage::Assistant(AssistantMessage {
                                    message: "".to_string(),
                                    is_done: true,
                                }),
                                stream,
                            )));
                        }
                    }
                }
            } else {
                return Ok(None);
            }
        }
        Ok(None)
    });

    Ok(Box::pin(stream))
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
enum ChatEndpointError {
    Auth,
    Internal(anyhow::Error),
}

#[cfg(feature = "ssr")]
impl From<ChatEndpointError> for StatusCode {
    fn from(error: ChatEndpointError) -> Self {
        match error {
            ChatEndpointError::Auth => StatusCode::UNAUTHORIZED,
            ChatEndpointError::Internal(e) => {
                tracing::error!("Internal error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn chat_endpoint(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(organization_id): Path<i64>,
    ws: ws::WebSocketUpgrade,
) -> Result<Response, StatusCode> {
    use models::{organization::Role, source::get_sources_for_user_id_and_organization_id};

    use crate::auth::utils::{
        get_logged_in_user_with_session_id_and_pool, get_session_id_from_headers,
    };

    let session_id = get_session_id_from_headers(&headers)
        .map_err(|e| {
            tracing::error!("{e:?}");
            ChatEndpointError::Internal(anyhow::anyhow!("internal server error"))
        })?
        .ok_or(ChatEndpointError::Auth)?;

    let user = get_logged_in_user_with_session_id_and_pool(session_id, &state.pool)
        .await
        .map_err(|e| {
            tracing::error!("{e:?}");
            ChatEndpointError::Internal(anyhow::anyhow!("internal server error"))
        })?
        .ok_or(ChatEndpointError::Auth)?;

    // Just need to verify they have Some organization role
    let _ = Role::get_with_user_id_and_organization_id(user.id, organization_id, &state.pool)
        .await
        .map_err(|e| {
            tracing::error!("{e:?}");
            ChatEndpointError::Internal(anyhow::anyhow!("internal server error"))
        })?
        .ok_or(ChatEndpointError::Auth)?;

    let sources =
        get_sources_for_user_id_and_organization_id(user.id, organization_id, &state.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                ChatEndpointError::Internal(anyhow::anyhow!("internal server error"))
            })?;

    Ok(ws.on_upgrade(move |socket| handle_socket(user.id, organization_id, sources, socket)))
}

#[cfg(feature = "ssr")]
pub async fn chat_endpoint_canned(
    State(state): State<AppState>,
    ws: ws::WebSocketUpgrade,
) -> Result<Response, StatusCode> {
    use models::source::get_sources_for_user_id_and_organization_id;

    let user_id: i64 = std::env::var("SUNDRY_DEMO_CANNED_USER_ID")
        .expect("missing SUNDRY_DEMO_CANNED_USER_ID env variable")
        .parse()
        .expect("SUNDRY_DEMO_CANNED_USER_ID is not a valid i64");
    let organization_id: i64 = std::env::var("SUNDRY_DEMO_CANNED_USER_ORGANIZATION_ID")
        .expect("missing SUNDRY_DEMO_CANNED_USER_ORGANIZATION_ID env variable")
        .parse()
        .expect("SUNDRY_DEMO_CANNED_USER_ORGANIZATION_ID is not a valid i64");

    let sources =
        get_sources_for_user_id_and_organization_id(user_id, organization_id, &state.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                ChatEndpointError::Internal(anyhow::anyhow!("internal server error"))
            })?;

    Ok(ws.on_upgrade(move |socket| handle_socket(user_id, organization_id, sources, socket)))
}

#[cfg(feature = "ssr")]
pub async fn handle_socket(
    user_id: i64,
    organization_id: i64,
    sources: Sources,
    mut socket: ws::WebSocket,
) {
    socket
        .send(WebSocketResponse::Sources(sources.clone()).into())
        .await
        .ok();

    // We don't need to handle every error in here if we have a socket recv error or
    // an error converting the msg to text that isn't an error on our servers part
    // but on the clients
    while let Some(Ok(msg)) = socket.recv().await {
        if let ws::Message::Text(txt) = msg {
            if let Err(e) =
                handle_message(&txt, user_id, organization_id, &sources, &mut socket).await
            {
                tracing::error!("chat demo - handeling user message: {e:?}");
                socket
                    .send(
                        WebSocketResponse::ChatError("error during llm request".to_string()).into(),
                    )
                    .await
                    .ok();
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn handle_message(
    message: &str,
    user_id: i64,
    organization_id: i64,
    sources: &Sources,
    socket: &mut ws::WebSocket,
) -> anyhow::Result<()> {
    let mut messages = serde_json::from_str::<Vec<ChatMessage>>(message)?;

    let mut break_out = false;
    while !break_out {
        // Set break_out to true
        // We only want to stream from the llm once unless we call a tool
        break_out = true;

        let message_id = messages.len() as u32;
        let messages_clone = messages.clone();
        let mut stream = create_chat_stream(&messages_clone, sources).await?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(StreamResponseMessage::ToolCall(tool_call)) => {
                    let tool_call_message = ChatMessage {
                        id: message_id,
                        content: MessageContent::ToolCall(tool_call.clone()),
                    };
                    messages.push(tool_call_message.clone());
                    socket
                        .send(WebSocketResponse::Success(tool_call_message).into())
                        .await
                        .ok();

                    let arguments: GetContextToolArguments =
                        serde_json::from_str(&tool_call.arguments)?;
                    let context = get_context(&arguments.query, user_id, organization_id).await?;

                    let tool_response_message = ChatMessage {
                        id: message_id + 1,
                        content: MessageContent::ToolResponse(ToolResponseMessage {
                            id: tool_call.id,
                            content: context,
                        }),
                    };
                    messages.push(tool_response_message.clone());
                    socket
                        .send(WebSocketResponse::Success(tool_response_message).into())
                        .await
                        .ok();

                    break_out = false;
                    break;
                }
                Ok(StreamResponseMessage::Assistant(assistant)) => {
                    socket
                        .send(
                            WebSocketResponse::Success(ChatMessage {
                                id: message_id,
                                content: MessageContent::Assistant(assistant),
                            })
                            .into(),
                        )
                        .await
                        .ok();
                }
                Err(e) => {
                    anyhow::bail!(e)
                }
            }
        }
    }

    Ok(())
}

#[component]
fn AssistantMessage(children: Children) -> impl IntoView {
    view! {
        <div class="flex justify-start">
            <div class="bg-neutral-700 text-neutral-200 p-3 rounded-md max-w-[80%]">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn Message(message_group: Vec<ChatMessage>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 p-4">
            {match message_group.first().map(|message| &message.content) {
                Some(MessageContent::User(user_message)) => {
                    view! {
                        <div class="flex justify-end">
                            <div class="bg-neutral-200 text-black p-3 rounded-md max-w-[80%]">
                                {user_message.message.clone()}
                            </div>
                        </div>
                    }
                        .into_any()
                }
                Some(_) => {
                    let messages_length = message_group.len();
                    view! {
                        // If messages_length -1 == message.0 this is the last message in the group
                        <AssistantMessage>
                            <For
                                each=move || {
                                    message_group
                                        .clone()
                                        .into_iter()
                                        .enumerate()
                                        .collect::<Vec<(usize, ChatMessage)>>()
                                }
                                key=|message| message.1.key()
                                children=move |message| {
                                    match message.1.content {
                                        MessageContent::Assistant(assistant_msg) => {
                                            view! {
                                                <div
                                                    class="demo-assistant-message"
                                                    inner_html=markdown::to_html(&assistant_msg.message)
                                                ></div>
                                            }
                                                .into_any()
                                        }
                                        MessageContent::ToolCall(_)
                                        | MessageContent::ToolResponse(_) => {
                                            view! {
                                                <div class="p-2 bg-neutral-600 mb-3">
                                                    {match &message.1.content {
                                                        MessageContent::ToolCall(tool_msg) => {
                                                            let val: serde_json::Value = serde_json::from_str(
                                                                    &tool_msg.arguments,
                                                                )
                                                                .unwrap_or_default();
                                                            view! {
                                                                <div>
                                                                    <div class="font-bold">"Tool Call"</div>
                                                                    <pre class="text-sm overflow-x-scroll">
                                                                        {serde_json::to_string_pretty(&val).unwrap()}
                                                                    </pre>
                                                                </div>
                                                            }
                                                                .into_any()
                                                        }
                                                        MessageContent::ToolResponse(tool_response) => {
                                                            let val: serde_json::Value = serde_json::from_str(
                                                                    &tool_response.content,
                                                                )
                                                                .unwrap_or_default();
                                                            view! {
                                                                <div class="">
                                                                    <div class="font-bold">"Tool Response"</div>
                                                                    <pre class="text-sm overflow-x-scroll">
                                                                        {serde_json::to_string_pretty(&val).unwrap()}
                                                                    </pre>
                                                                </div>
                                                            }
                                                                .into_any()
                                                        }
                                                        _ => unreachable!(),
                                                    }}
                                                    {(messages_length - 1 == message.0)
                                                        .then(move || {
                                                            view! {
                                                                <div class="flex items-center gap-2 pt-3">
                                                                    <LoadingWheel size=LoadingWheelSize::Small />
                                                                    <span>
                                                                        {match message.1.content {
                                                                            MessageContent::ToolCall(_) => "Calling Tool...",
                                                                            MessageContent::ToolResponse(_) => "Calling LLM...",
                                                                            _ => unreachable!(),
                                                                        }}
                                                                    </span>
                                                                </div>
                                                            }
                                                        })}
                                                </div>
                                            }
                                                .into_any()
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                            />
                        </AssistantMessage>
                    }
                        .into_any()
                }
                None => unreachable!(),
            }}
        </div>
    }
}

#[component]
pub fn CannedDemo() -> impl IntoView {
    // The organization_id does not matter here
    view! { <Demo organization_id=0 is_canned=true /> }
}

#[component]
pub fn LoggedInDemo() -> impl IntoView {
    let organization: Organization = expect_context();

    view! { <Demo organization_id=organization.id is_canned=false /> }
}

#[component]
pub fn Demo(organization_id: i64, is_canned: bool) -> impl IntoView {
    let (is_connected, set_is_connected) = signal(false);
    let (messages, set_messages) = signal::<Vec<ChatMessage>>(vec![]);
    let (is_users_turn, set_is_users_turn) = signal(true);
    let (is_waiting_for_initial_response, set_is_waiting_for_initial_response) = signal(false);
    let (input_value, set_input_value) = signal(String::new());
    let (is_no_api_key_error, set_is_no_api_key_error) = signal(false);
    let (sources, set_sources) = signal(None);

    let (show_info_modal, set_show_info_modal) = signal(true);

    let messages_container = NodeRef::<Div>::new();
    let (messages_are_scrolled_to_bottom, set_messages_are_scrolled_to_bottom) = signal(true);

    let add_message = move |message: ChatMessage| {
        let messages = messages.get_untracked();
        if let Some(last_message) = messages.last() {
            if last_message.id == message.id {
                set_messages.update(|messages| {
                    messages.last_mut().unwrap().merge(message);
                });
            } else {
                set_messages.update(|messages| {
                    messages.push(message);
                });
            }
        } else {
            set_messages.set(vec![message]);
        }
    };

    let (tx, rx) = mpsc::channel::<String>(100);
    let rx_signal = RwSignal::new(rx);

    let connect_ws = move || {
        let websocket_endpoint = if is_canned {
            if *PUBLIC_DEV_MODE {
                "ws://127.0.0.1:3000/demo-ws-canned".to_string()
            } else {
                "wss://getsundry.app/demo-ws-canned".to_string()
            }
        } else if *PUBLIC_DEV_MODE {
            format!("ws://127.0.0.1:3000/dashboard/{organization_id}/demo-ws")
        } else {
            format!("wss://getsundry.app/dashboard/{organization_id}/demo-ws")
        };

        match WebSocket::new(&websocket_endpoint) {
            Ok(ws) => {
                // Open event
                let onopen_callback = Closure::<dyn FnMut()>::new(move || {
                    log!("WebSocket connection opened");
                    set_is_connected.set(true);
                });
                ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                onopen_callback.forget();

                // Message event
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    match e.data().as_string() {
                        Some(message) => {
                            set_is_waiting_for_initial_response.set(false);
                            match serde_json::from_str::<WebSocketResponse>(&message) {
                                Ok(message) => match message {
                                    WebSocketResponse::ChatError(error_message) => {
                                        window()
                                            .alert_with_message(&format!(
                                                "Error getting llm response: {error_message} - Please reload the page"
                                            ))
                                            .ok();
                                    }
                                    WebSocketResponse::NoAPIKeys => {
                                        set_is_no_api_key_error.set(true);
                                    }
                                    WebSocketResponse::Sources(sources) => {
                                        set_sources.set(Some(sources));
                                    }
                                    WebSocketResponse::Success(message) => {
                                        if let MessageContent::Assistant(assistant_message) =
                                            &message.content
                                        {
                                            if assistant_message.is_done {
                                                set_is_users_turn.set(true);
                                            }
                                        }
                                        add_message(message);
                                    }
                                },
                                Err(_) => {
                                    log!("error parsing websocket string into message: {e:?}");
                                }
                            }
                        }
                        None => {
                            log!("error parsing websocket message to string: {e:?}")
                        }
                    };
                });
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();

                // Close event
                let onclose_callback = Closure::<dyn FnMut(_)>::new(move |_: CloseEvent| {
                    log!("WebSocket connection closed");
                    // set_is_connected.set(false);
                });
                ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
                onclose_callback.forget();

                // Error event
                let onerror_callback =
                    Closure::<dyn FnMut(ErrorEvent)>::new(move |e: ErrorEvent| {
                        log!("WebSocket error: {:?}", e);
                        // set_is_connected.set(false);
                    });
                ws.set_onclose(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();

                // Setting the is_users_turn to true here will prevent locks if an error occurs
                // while waiting for a response from the server
                set_is_users_turn.set(true);

                spawn_local(async move {
                    let mut rx = rx_signal.write_untracked();
                    while let Some(msg) = (*rx).next().await {
                        let message_id = messages.get_untracked().len();
                        let message = ChatMessage::new_user_message(message_id as u32, &msg);
                        set_messages.update(|messages| messages.push(message.clone()));
                        if let Err(err) = ws.send_with_str(
                            &serde_json::to_string(&messages.get_untracked()).unwrap(),
                        ) {
                            log!("Error sending message: {:?}", err);
                        } else {
                            set_is_users_turn.set(false);
                            set_is_waiting_for_initial_response.set(true);
                        }
                    }
                });
            }
            Err(err) => {
                log!("Failed to create WebSocket: {:?}", err);
            }
        }
    };

    // Effect to maintain connection
    // TODO: Set some timeout delay reconnection attempts
    Effect::new(move |_| {
        if !is_connected.get() {
            log!("Attempting to connect WebSocket...");
            connect_ws();
        }
    });

    // Auto-scroll effect when streaming or a new message is added
    Effect::new(move |_| {
        // Watch for message changes
        messages.get();

        if messages_are_scrolled_to_bottom.get() {
            if let Some(container) = messages_container.get() {
                container.scroll_to_with_x_and_y(0.0, container.scroll_height() as f64);
            }
        }
    });

    // Group the assistant tool call, tool response, and final response messages
    // together We essentially create "message groups"
    // User messages are always a single group
    // Assistant message groups are either a single assistant message or start with
    // a tool call, are composed of any number of tool calls and responses and end
    // with an assistant message
    let group_messages = |messages: Vec<ChatMessage>| {
        messages.into_iter().fold(vec![], |mut acc, message| {
            match (acc.last_mut(), &message.content) {
                (_, MessageContent::User(_)) => acc.push(vec![message]),
                (None, MessageContent::Assistant(_)) => acc.push(vec![message]),
                (None, MessageContent::ToolCall(_)) => acc.push(vec![message]),
                (Some(message_group), MessageContent::Assistant(_)) => {
                    match message_group.last().map(|m| &m.content) {
                        // If the last message in the last group was a user or assistant we start a
                        // new message group
                        Some(MessageContent::User(_)) | Some(MessageContent::Assistant(_)) => {
                            acc.push(vec![message])
                        }
                        // If the last message in the last group was a tool call or tool response we
                        // know this message is part of that group
                        Some(MessageContent::ToolCall(_))
                        | Some(MessageContent::ToolResponse(_)) => message_group.push(message),
                        None => unreachable!(),
                    }
                }
                (Some(message_group), MessageContent::ToolCall(_)) => {
                    match message_group.last().map(|m| &m.content) {
                        // If the last message in the last group was a user or assistant we start a
                        // new message group
                        Some(MessageContent::User(_)) | Some(MessageContent::Assistant(_)) => {
                            acc.push(vec![message])
                        }
                        // If the last message in the last group was a tool call or tool response we
                        // know this message is part of that group
                        Some(MessageContent::ToolCall(_))
                        | Some(MessageContent::ToolResponse(_)) => message_group.push(message),
                        None => unreachable!(),
                    }
                }
                // Tool responses are never the start of a group
                (Some(message_group), MessageContent::ToolResponse(_)) => {
                    message_group.push(message)
                }
                // This one should be impossible - assistant responses never start with a tool
                // response
                (None, MessageContent::ToolResponse(_)) => (),
            }
            acc
        })
    };

    let questions = [
        "What was the last GitHub issue assigned to me?",
        "Who all commented on my last Pull Request?",
        "Do I have any issues labeled as bugs assigned to me?",
        "List all GitHub issues currently open and assigned to me",
    ];

    let tx = StoredValue::new(tx.clone());
    view! {
        <Modal
            show=(show_info_modal, set_show_info_modal)
            show_close_button=true
            allow_close=Signal::derive(move || {
                sources.get().map(|sources| !sources.is_empty()).unwrap_or_default()
            })
        >
            <h6 class="mb-2">Sundry demo</h6>
            <div class="mb-3">
                {if is_canned {
                    view! { <div>"some info about how this is canned"</div> }.into_any()
                } else {
                    view! {
                        <div>
                            "Welcome to the Sundry Demo. Here you can chat with OpenAI about your data. This demo serves as a simple example of the kind of applications you can make with Sundry."
                        </div>
                        <div class="mt-2">
                            "Note: It may take up to 10 minutes for new connected data to ingest. Thank you for your patience!"
                        </div>
                    }
                        .into_any()
                }}
            </div>

            {move || {
                let sources = sources.get();
                if sources.is_some() {
                    let sources = sources.unwrap();
                    if !sources.is_empty() {
                        view! {
                            <div class="font-bold mb-2">"Available sources:"</div>
                            <pre class="bg-black px-4 py-3">
                                {move || { serde_json::to_string_pretty(&sources).unwrap() }}
                            </pre>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div>
                                <A href=format!("/dashboard/{organization_id}/overview")>
                                    <span class="mt-3 inline-block bg-neon-shade-100 py-2 px-3 text-black rounded-sm">
                                        "Connect Data to Use the Demo"
                                    </span>
                                </A>
                            </div>
                        }
                            .into_any()
                    }
                } else {
                    view! { <div>"Loading available sources..."</div> }.into_any()
                }
            }}
        </Modal>

        <Modal show=(is_no_api_key_error, set_is_no_api_key_error)>
            <div>Link to generate API keys</div>
        </Modal>

        <div
            class="demo-container flex flex-col no-scrollbar overflow-y-scroll"
            class:justify-center=move || messages.get().is_empty()
        >
            {move || {
                if !messages.get().is_empty() {
                    view! {
                        // Message container
                        <div
                            class="flex-1 overflow-y-auto pl-10 pr-10"
                            node_ref=messages_container
                            on:scroll=move |ev| {
                                let target = event_target::<HtmlDivElement>(&ev);
                                let is_at_bottom = (target.scroll_height() - target.scroll_top()
                                    - target.client_height()) <= 0;
                                set_messages_are_scrolled_to_bottom.set(is_at_bottom);
                            }
                        >
                            <div class="w-full">
                                <For
                                    each=move || group_messages(messages.get())
                                    key=|message_group| {
                                        message_group
                                            .iter()
                                            .fold(
                                                "".to_string(),
                                                |mut acc, message| {
                                                    acc.push_str(&message.key());
                                                    acc
                                                },
                                            )
                                    }
                                    children=move |message_group| {
                                        view! { <Message message_group=message_group /> }
                                    }
                                />
                                {{
                                    move || {
                                        is_waiting_for_initial_response
                                            .get()
                                            .then(|| {
                                                view! {
                                                    <AssistantMessage>
                                                        <div class="flex items-center gap-2">
                                                            <LoadingWheel size=LoadingWheelSize::Small />
                                                            <span>"Calling LLM..."</span>
                                                        </div>
                                                    </AssistantMessage>
                                                }
                                            })
                                    }
                                }}
                            </div>
                        // Input container
                        </div>
                        <div class="border-neutral-600 px-10 py-5">
                            <div class="w=full">
                                <div
                                    class="flex gap-2 rounded-sm bg-neutral-700 p-2 border-b-2 border-neutral-700"
                                    class=(
                                        "focus-within:border-neon-shade-100",
                                        move || is_users_turn.get(),
                                    )
                                >
                                    <input
                                        type="text"
                                        class="w-full !p-0 !bg-none outline-none border-none focus:outline-none focus:border-none"
                                        prop:value=move || input_value.get()
                                        on:input=move |ev| {
                                            set_input_value.set(event_target_value(&ev))
                                        }
                                        on:keypress=move |ev| {
                                            if ev.key() == "Enter" {
                                                tx.with_value(|tx| {
                                                    let mut tx = tx.clone();
                                                    tx.try_send(input_value.get_untracked()).ok();
                                                    set_input_value.set("".to_string());
                                                });
                                            }
                                        }
                                        placeholder="What was the last GitHub issue assigned to me..."
                                    />
                                    <button
                                        class="text-neon-shade-100 focus:outline-none"
                                        disabled=move || !is_users_turn.get()
                                        on:click=move |_| {
                                            tx.with_value(|tx| {
                                                let mut tx = tx.clone();
                                                tx.try_send(input_value.get_untracked()).ok();
                                                set_input_value.set("".to_string());
                                            });
                                        }
                                    >
                                        <span class="material-symbols-outlined">play_arrow</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <div class="w-full px-10">
                            <h4 class="mb-6 text-center">Chat with your data</h4>
                            <div class="flex gap-2 mb-3">
                                {questions
                                    .into_iter()
                                    .map(|question| {
                                        view! {
                                            <div
                                                class="bg-neutral-800 border-2 border-neon-shade-800 p-2 cursor-pointer"
                                                on:click=move |_| {
                                                    tx.with_value(|tx| {
                                                        let mut tx = tx.clone();
                                                        tx.try_send(question.to_string()).ok();
                                                    });
                                                }
                                            >
                                                {question}
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                            <div class="w=full">
                                <div
                                    class="flex gap-2 rounded-sm bg-neutral-700 p-2 border-b-2 border-neutral-700"
                                    class=(
                                        "focus-within:border-neon-shade-100",
                                        move || is_users_turn.get(),
                                    )
                                >
                                    <input
                                        type="text"
                                        class="w-full !p-0 !bg-none outline-none border-none focus:outline-none focus:border-none"
                                        prop:value=move || input_value.get()
                                        on:input=move |ev| {
                                            set_input_value.set(event_target_value(&ev))
                                        }
                                        on:keypress=move |ev| {
                                            if ev.key() == "Enter" {
                                                tx.with_value(|tx| {
                                                    let mut tx = tx.clone();
                                                    tx.try_send(input_value.get_untracked()).ok();
                                                    set_input_value.set("".to_string());
                                                });
                                            }
                                        }
                                        placeholder="What was the last GitHub issue assigned to me..."
                                    />
                                    <button
                                        class="text-neon-shade-100 focus:outline-none"
                                        disabled=move || !is_users_turn.get()
                                        on:click=move |_| {
                                            tx.with_value(|tx| {
                                                let mut tx = tx.clone();
                                                tx.try_send(input_value.get_untracked()).ok();
                                                set_input_value.set("".to_string());
                                            });
                                        }
                                    >
                                        <span class="material-symbols-outlined">play_arrow</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                        .into_any()
                }
            }}

        </div>
    }
}
