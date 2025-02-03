#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
pub trait LogAndMapInternalServerError<T> {
    fn ise(self) -> Result<T, ServerFnError>;

    fn ise_with_user_message(self, user_message: &str) -> Result<T, ServerFnError>;
}

#[cfg(feature = "ssr")]
impl<T, E: std::fmt::Debug> LogAndMapInternalServerError<T> for Result<T, E> {
    fn ise(self) -> Result<T, ServerFnError> {
        self.ise_with_user_message("internal server error")
    }

    fn ise_with_user_message(self, user_message: &str) -> Result<T, ServerFnError> {
        self.map_err(|e| {
            tracing::error!("{e:?}");
            ServerFnError::new(user_message)
        })
    }
}
