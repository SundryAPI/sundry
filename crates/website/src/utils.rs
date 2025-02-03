use leptos::prelude::*;

pub fn get_server_fn_user_error(err: ServerFnError) -> Option<String> {
    match err {
        ServerFnError::Request(_) => {
            Some("Error making network request. Check your internet connection.".to_string())
        }
        ServerFnError::ServerError(e) => Some(format!("Error: {e}")),
        _ => Some("Unknown Error".to_string()),
    }
}

pub fn get_maybe_server_fn_user_error<T>(val: Option<Result<T, ServerFnError>>) -> Option<String> {
    val.map(|res| match res {
        Ok(_) => None,
        Err(e) => match e {
            ServerFnError::Request(_) => {
                Some("Error making network request. Check your internet connection.".to_string())
            }
            ServerFnError::ServerError(e) => Some(format!("Error: {e}")),
            _ => Some("Unknown Error".to_string()),
        },
    })
    .flatten()
}
