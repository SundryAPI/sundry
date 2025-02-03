use chrono;
use leptos::prelude::*;
use models::user::User;

#[server]
pub async fn add_user_from_invitation(
    user: User,
    invite_secret: String,
) -> Result<(), ServerFnError> {
    use crate::{error::LogAndMapInternalServerError, AppState};
    use anyhow::Context;
    let state: AppState = expect_context();
    if let Some(invite) =
        models::organization::OrganizationInvitation::get_with_secret(invite_secret, &state.pool)
            .await
            .ise()?
    {
        if chrono::Utc::now().naive_utc() > invite.created_at + chrono::Duration::days(5) {
            return Err(ServerFnError::new("invite expired"));
        }

        let organization =
            models::organization::Organization::get_with_id(invite.organization_id, &state.pool)
                .await
                .ise()?
                .context("getting organization")
                .ise()?;

        let mut conn = state.pool.acquire().await.ise()?;

        if let Ok(_) = organization
            .add_user(&user, models::organization::Role::Admin, &mut conn)
            .await
            .ise()
        {
            let _ = invite.delete(&state.pool).await;
        }
    } else {
        return Err(ServerFnError::new("no invite found"));
    };
    Ok(())
}
