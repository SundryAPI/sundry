use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct User {
    user: models::user::User,
    refetch_fn: Arc<dyn Fn() -> () + Send + Sync + 'static>,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("user", &self.user)
            .field("refetch_fn", &"<function>")
            .finish()
    }
}

impl User {
    pub fn new(
        user: models::user::User,
        refetch_fn: Arc<dyn Fn() -> () + Send + Sync + 'static>,
    ) -> Self {
        Self { user, refetch_fn }
    }

    pub fn refresh(&self) {
        (self.refetch_fn)()
    }

    pub fn get_user(&self) -> &models::user::User {
        &self.user
    }
}
