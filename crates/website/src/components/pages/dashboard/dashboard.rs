use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route},
    hooks::use_params_map,
    path, MatchNestedRoutes,
};
use models::organization::{Organization, Role};

#[server(GetOrganization, "/api")]
pub async fn get_organization(organization_id: i64) -> Result<Option<Organization>, ServerFnError> {
    use crate::auth::utils::get_logged_in_user;
    use crate::AppState;

    let user = get_logged_in_user()
        .await?
        .ok_or(ServerFnError::new("not authorized"))?;

    let state: AppState = use_context().ok_or(ServerFnError::new("internal server error"))?;

    Ok(Organization::get_with_id_and_user(organization_id, user.id, &state.pool).await?)
}

#[server(GetUserRole, "/api")]
pub async fn get_user_role(organization_id: i64) -> Result<Option<Role>, ServerFnError> {
    use crate::auth::utils::get_logged_in_user_organization_role;

    let role = get_logged_in_user_organization_role(organization_id)
        .await
        .ok()
        .flatten();

    Ok(role)
}

#[component]
pub fn OrganizationGate() -> impl IntoView {
    let params = use_params_map();
    let organization_resource = Resource::new(
        move || (),
        move |_| async move {
            match params.get().get("id") {
                Some(id) => get_organization(id.parse()?).await,
                None => Ok(None),
            }
        },
    );

    let user_role = Resource::new(
        move || (),
        move |_| async move {
            match params.get().get("id") {
                Some(id) => get_user_role(id.parse()?).await,
                None => Ok(None),
            }
        },
    );

    // Currently using workaround: https://github.com/leptos-rs/leptos/issues/3042
    let outer_owner = Owner::current().unwrap();
    let inner_view = move || {
        Suspend::new({
            let outer_owner = outer_owner.clone();
            async move {
                match (
                    organization_resource.await.ok().flatten(),
                    user_role.await.ok().flatten(),
                ) {
                    (Some(organization), Some(user_role)) => {
                        outer_owner.with(|| {
                            provide_context(organization);
                            provide_context(user_role);
                        });
                        Outlet()
                    }
                    .into_any(),
                    _ => view! { <Redirect path="/dashboard" /> }.into_any(),
                }
            }
        })
    };

    // Using Transition workaround: https://github.com/leptos-rs/leptos/issues/2907
    view! {
        <Transition fallback=move || {
            view! { <p>"Loading Organization..."</p> }
        }>{inner_view}</Transition>
    }
}

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("") view=Outlet>
            <Route path=path!("") view=Dashboard />

            <ParentRoute path=path!(":id") view=OrganizationGate>
                <Route path=path!(":tab") view=super::organization::index::Index />
            </ParentRoute>
        </ParentRoute>
    }
    .into_inner()
}

#[server]
pub async fn get_organizations() -> Result<Vec<Organization>, ServerFnError> {
    use crate::auth::utils::get_logged_in_user;
    use crate::AppState;

    let user = get_logged_in_user()
        .await?
        .ok_or(ServerFnError::new("not authorized"))?;

    let state: AppState = use_context().ok_or(ServerFnError::new("internal server error"))?;

    Ok(Organization::get_with_user(user.id, &state.pool).await?)
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let organizations = Resource::new(
        move || (),
        move |_| async move { get_organizations().await },
    );

    view! {
        <Show when=move || organizations.get().is_some() fallback=move || view! { Loading Orgs... }>
            <Redirect path=format!(
                "/dashboard/{}/overview",
                organizations.get().unwrap().unwrap().first().unwrap().id,
            ) />
        </Show>
    }
}
