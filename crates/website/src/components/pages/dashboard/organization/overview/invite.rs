use crate::components::atoms::button::{Button, ButtonVariant};
use crate::components::atoms::dropdown::{Dropdown, DropdownItem};
use crate::components::atoms::table::{Row, RowType, Table};
use crate::components::atoms::tag::{Tag, TagVariant};
use crate::components::molecules::modal::Modal;
use crate::email::{Email, SendgridTemplate};
use leptos::context::use_context;
use leptos::prelude::*;
use models::organization::{Organization, OrganizationInvitation, Role};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TeamMemberStatus {
    Active,
    Pending,
}

impl TeamMemberStatus {
    pub fn to_tag_variant(&self) -> TagVariant {
        match self {
            TeamMemberStatus::Active => TagVariant::Active,
            TeamMemberStatus::Pending => TagVariant::Invited,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TeamMember {
    pub email: String,
    pub role: Role,
    pub status: TeamMemberStatus,
    pub id: String,
}

#[server(InviteUser, "/dashboard/:id/api")]
pub async fn invite_user(email: String, org_id: i64, role: String) -> Result<(), ServerFnError> {
    use crate::{auth::utils::get_logged_in_user, error::LogAndMapInternalServerError};
    use anyhow::Context;
    let state: crate::AppState = expect_context();

    let user = get_logged_in_user()
        .await
        .ise()?
        .context("user not logged in")
        .ise()?;

    let role = Role::try_from(role).ise()?;

    let organization = Organization::get_with_id(org_id, &state.pool)
        .await
        .ise()?
        .ok_or(ServerFnError::new("organization not found"))
        .ise()?;

    organization
        .user_is_owner(user.id, &state.pool)
        .await
        .is_ok_and(|x| x)
        .then(|| ())
        .ok_or(ServerFnError::new("user is not owner"))
        .ise()?;

    let new_invite = OrganizationInvitation::create(organization.id, &email, role, &state.pool)
        .await
        .ise()?;

    let invite_link = format!(
        "https://getsundry.app/signup?invite_secret={}",
        new_invite.secret
    );

    let email = Email {
        email: &email,
        name: None,
        content: Some(&invite_link),
        template: SendgridTemplate::InviteUser,
    };

    let _ = email.send().await.ise()?;

    Ok(())
}

#[server(DeleteInvite)]
pub async fn delete_invite(invite_secret: String) -> Result<(), ServerFnError> {
    use crate::{auth::utils::get_logged_in_user, error::LogAndMapInternalServerError};
    use anyhow::Context;
    let state: crate::AppState = expect_context();

    let user = get_logged_in_user()
        .await
        .ise()?
        .context("user not logged in")
        .ise()?;

    let invite = OrganizationInvitation::get_with_secret(invite_secret, &state.pool)
        .await
        .ise()?
        .ok_or(ServerFnError::new("invite not found"))
        .ise()?;

    let organization = Organization::get_with_id(invite.organization_id, &state.pool)
        .await
        .ise()?
        .ok_or(ServerFnError::new("organization not found"))
        .ise()?;

    organization
        .user_is_owner(user.id, &state.pool)
        .await
        .is_ok_and(|x| x)
        .then(|| ())
        .ok_or(ServerFnError::new("user is not owner"))
        .ise()?;

    invite.delete(&state.pool).await.ise()?;

    Ok(())
}

#[server(ResendInvite)]
pub async fn resend_invite(invite_secret: String) -> Result<(), ServerFnError> {
    use crate::{auth::utils::get_logged_in_user, error::LogAndMapInternalServerError};
    use anyhow::Context;
    let state: crate::AppState = expect_context();

    let user = get_logged_in_user()
        .await
        .ise()?
        .context("user not logged in")
        .ise()?;

    let invite = OrganizationInvitation::get_with_secret(invite_secret, &state.pool)
        .await
        .ise()?
        .ok_or(ServerFnError::new("invite not found"))
        .ise()?;

    let organization = Organization::get_with_id(invite.organization_id, &state.pool)
        .await
        .ise()?
        .ok_or(ServerFnError::new("organization not found"))
        .ise()?;

    organization
        .user_is_owner(user.id, &state.pool)
        .await
        .is_ok_and(|x| x)
        .then(|| ())
        .ok_or(ServerFnError::new("user is not owner"))
        .ise()?;

    let invite_link = format!(
        "https://getsundry.app/signup?invite_secret={}",
        invite.secret
    );

    let email = Email {
        email: &invite.email,
        name: None,
        content: Some(&invite_link),
        template: SendgridTemplate::InviteUser,
    };

    let _ = email.send().await.ise()?;

    Ok(())
}

#[server(GetTeam)]
pub async fn get_team(org_id: i64) -> Result<Vec<TeamMember>, ServerFnError> {
    use crate::{auth::utils::get_logged_in_user, error::LogAndMapInternalServerError};
    use anyhow::Context;
    let state: crate::AppState = expect_context();

    let user = get_logged_in_user()
        .await
        .ise()?
        .context("user not logged in")
        .ise()?;

    let organization = Organization::get_with_id(org_id, &state.pool)
        .await
        .ise()?
        .ok_or(ServerFnError::new("organization not found"))
        .ise()?;

    organization
        .user_is_owner(user.id, &state.pool)
        .await
        .is_ok_and(|x| x)
        .then(|| ())
        .ok_or(ServerFnError::new("user is not owner"))
        .ise()?;

    let organization_id = organization.id.clone();

    let mut team: Vec<TeamMember> = organization
        .get_all_organization_users(&state.pool)
        .await
        .ise()?
        .into_iter()
        .map(|user| TeamMember {
            email: user.email,
            role: user.role,
            status: TeamMemberStatus::Active,
            id: user.id.to_string(),
        })
        .collect();

    OrganizationInvitation::get_all_for_organization(organization_id, &state.pool)
        .await
        .ise()?
        .into_iter()
        .for_each(|invite| {
            team.push(TeamMember {
                email: invite.email,
                role: invite.role,
                status: TeamMemberStatus::Pending,
                id: invite.secret,
            })
        });

    Ok(team)
}

#[server(RemoveUser)]
pub async fn remove_user(user_id: String, org_id: i64) -> Result<(), ServerFnError> {
    use crate::{auth::utils::get_logged_in_user, error::LogAndMapInternalServerError};
    use anyhow::Context;
    let state: crate::AppState = expect_context();
    let user_id = user_id.parse::<i64>().ise()?;

    let user = get_logged_in_user()
        .await
        .ise()?
        .context("user not logged in")
        .ise()?;

    if user.id == user_id {
        return Err(ServerFnError::new("cannot remove yourself"));
    }

    let organization = Organization::get_with_id(org_id, &state.pool)
        .await
        .ise()?
        .ok_or(ServerFnError::new("organization not found"))
        .ise()?;

    organization
        .user_is_owner(user.id, &state.pool)
        .await
        .is_ok_and(|x| x)
        .then(|| ())
        .ok_or(ServerFnError::new("user is not owner"))
        .ise()?;

    let owner = organization
        .clone()
        .get_original_owner(&state.pool)
        .await
        .ise()?;

    if owner.id == user_id {
        return Err(ServerFnError::new("cannot remove original owner"));
    }

    organization.remove_user(user_id, &state.pool).await.ise()?;

    Ok(())
}

#[component]
pub fn InviteUser() -> impl IntoView {
    let invite_user = ServerAction::<InviteUser>::new();
    let (show_modal, set_show_modal) = signal(false);
    let (invite_role, set_invite_role) = signal(Role::Admin);
    let (invite_success, set_invite_success) = signal(false);
    let org = use_context::<Organization>().unwrap().id;

    let remove_user = Action::new(|input: &(String, i64)| {
        let user_id = input.0.to_owned();
        let org = input.1.to_owned();
        async move { remove_user(user_id.to_string(), org).await }
    });

    let delete_invite = Action::new(|invite_secret: &String| {
        let invite_secret = invite_secret.to_owned();
        async move { delete_invite(invite_secret).await }
    });

    let resend_invite = Action::new(|invite_secret: &String| {
        let invite_secret = invite_secret.to_owned();
        async move { resend_invite(invite_secret).await }
    });

    let team = Resource::new(
        move || {
            (
                remove_user.version().get(),
                delete_invite.version().get(),
                invite_user.value().get(),
            )
        },
        move |_| async move {
            let team = get_team(org).await.ok();
            team
        },
    );

    Effect::watch(
        move || invite_user.value().get(),
        move |val, _, _| {
            if val.is_some() {
                set_invite_success.set(true);
            }
        },
        false,
    );

    view! {
        <div class="flex flex-col gap-9">
            <div class="flex flex-row justify-between">
                <div class="flex flex-col gap-9">
                    <h3 class="flex gap-2 items-center">
                        <span class="material-symbols-outlined text-cornflower-tint-100 !text-5xl">
                            add
                        </span>
                        "Invite a new team member"
                    </h3>
                    <p>"Invite team members to use the organizations data sources."</p>
                </div>
                {move || {
                    let org = use_context::<Organization>().unwrap().id;
                    view! {
                        <div class="flex flex-col gap-4">
                            <div class="my-auto">
                                <Button
                                    variant=ButtonVariant::Primary
                                    on_click=Box::new(move || {
                                        set_show_modal.set(true);
                                    })
                                    attr:r#type="button"
                                >
                                    "Invite"
                                </Button>
                            </div>
                            <Modal
                                show=(show_modal, set_show_modal)
                                on_close=Box::new(move || {
                                    set_invite_success.set(false);
                                })
                            >
                                {move || {
                                    if invite_success.get() {
                                        view! { <div class="text-center">"Invite sent!"</div> }
                                            .into_any()
                                    } else {
                                        view! {
                                            <div class="flex flex-col gap-4">
                                                <h1 class="text-2xl">"Invite a new team member"</h1>
                                                <ActionForm action=invite_user>
                                                    <div class="flex flex-col gap-4">
                                                        <label class="flex flex-col items-start gap-2 w-100">
                                                            "Email"
                                                            <input type="email" class="auth w-full" name="email" />
                                                            <input type="hidden" name="org_id" value=org />
                                                        </label>
                                                        <label class="flex flex-col items-start gap-2 w-100">
                                                            "Role"
                                                            <input
                                                                type="hidden"
                                                                name="role"
                                                                value=move || invite_role.get().to_string()
                                                            />
                                                            {move || {
                                                                let role = invite_role.get().to_string();
                                                                view! {
                                                                    <div class="w-full">
                                                                        <Dropdown label=role>
                                                                            <DropdownItem on_click=Box::new(move || {
                                                                                set_invite_role.set(Role::Admin);
                                                                            })>
                                                                                <div class="w-full text-start">
                                                                                    {Role::Admin.to_string()}
                                                                                </div>
                                                                            </DropdownItem>
                                                                            <DropdownItem on_click=Box::new(move || {
                                                                                set_invite_role.set(Role::Owner);
                                                                            })>
                                                                                <div class="w-full text-start">
                                                                                    {Role::Owner.to_string()}
                                                                                </div>
                                                                            </DropdownItem>
                                                                        </Dropdown>
                                                                    </div>
                                                                }
                                                            }}
                                                        </label>
                                                        <div class="flex items-center justify-end gap-4">
                                                            <Button
                                                                variant=ButtonVariant::Secondary
                                                                attr:r#type="button"
                                                                on_click=Box::new(move || {
                                                                    set_show_modal.set(false);
                                                                })
                                                            >
                                                                "Close"
                                                            </Button>
                                                            <Button
                                                                variant=ButtonVariant::Secondary
                                                                attr:r#type="submit"
                                                            >
                                                                "Invite"
                                                            </Button>
                                                        </div>
                                                    </div>
                                                </ActionForm>
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }}

                            </Modal>
                        </div>
                    }
                }}
            </div>

            <Suspense fallback=move || {
                view! {}
            }>
                <div class="py-5">
                    <Table>
                        <Row _type=RowType::Header>
                            <p class="ms-6">Email</p>
                            <p>Role</p>
                            <p>Status</p>
                            <p class="me-6">Actions</p>
                        </Row>
                        {move || {
                            match team.get().flatten() {
                                Some(team) => {
                                    team.iter()
                                        .map(|user| {
                                            let email = user.email.clone();
                                            let id = user.id.to_string();
                                            let role = user.role.to_string();
                                            let status_type = user.status.to_tag_variant();
                                            let action = match user.status {
                                                TeamMemberStatus::Active => {
                                                    view! {
                                                        <Dropdown label=Default::default()>
                                                            <DropdownItem on_click=Box::new(move || {
                                                                remove_user.dispatch((id.clone(), org));
                                                            })>
                                                                <div class="w-full text-start">"Remove user"</div>
                                                            </DropdownItem>
                                                        </Dropdown>
                                                    }
                                                        .into_any()
                                                }
                                                TeamMemberStatus::Pending => {
                                                    let resend_id = id.clone();
                                                    let delete_id = id.clone();
                                                    view! {
                                                        <Dropdown label=Default::default()>
                                                            <DropdownItem on_click=Box::new(move || {
                                                                resend_invite.dispatch(resend_id.clone());
                                                            })>
                                                                <div class="w-full text-start">"Resend invite"</div>
                                                            </DropdownItem>
                                                            <DropdownItem on_click=Box::new(move || {
                                                                delete_invite.dispatch(delete_id.clone());
                                                            })>
                                                                <div class="w-full text-start">"Delete invite"</div>
                                                            </DropdownItem>
                                                        </Dropdown>
                                                    }
                                                        .into_any()
                                                }
                                            };
                                            view! {
                                                <Row>
                                                    <p class="ms-6">{email}</p>
                                                    <p>{role}</p>
                                                    <Tag variant=status_type />
                                                    <div class="w-fit me-6">{action}</div>
                                                </Row>
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                                _ => view! {}.into_any(),
                            }
                        }}
                    </Table>
                </div>
            </Suspense>

        </div>
    }
}
