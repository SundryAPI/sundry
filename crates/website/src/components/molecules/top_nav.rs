use leptos::prelude::*;

use leptos::leptos_dom::helpers::window;
use models::organization::Organization;

use crate::components::atoms::{
    button::{Button, ButtonVariant},
    dropdown::{Dropdown, DropdownItem},
};
use crate::email::{Email, SendgridTemplate};

#[server(LogoutUser, "/api")]
pub async fn logout_user() -> Result<bool, ServerFnError> {
    use crate::{auth::utils::logout_user, error::LogAndMapInternalServerError, AppState};

    let state: AppState = expect_context();

    logout_user(&state.pool).await.ise()?;
    Ok(true)
}

#[server(SubmitFeedback, "/api")]
pub async fn submit_feedback(feedback: String) -> Result<(), ServerFnError> {
    use crate::{auth::utils::get_logged_in_user, error::LogAndMapInternalServerError};

    let user = get_logged_in_user()
        .await
        .ise()?
        .ok_or(ServerFnError::new("No user logged in"))
        .ise()?;

    let email = Email {
        email: &user.email,
        name: user.name.as_deref(),
        content: Some(&feedback),
        template: SendgridTemplate::Feedback,
    };

    let _ = email.send().await.ise()?;

    Ok(())
}

#[component]
pub fn Feedback(show: ReadSignal<bool>, set_show: WriteSignal<bool>) -> impl IntoView {
    let (success_message, set_success_message) = signal(None::<bool>);
    let submit_feedback = ServerAction::<SubmitFeedback>::new();

    Effect::watch(
        move || submit_feedback.value().get(),
        move |val, _, _| {
            if let Some(val) = val {
                match val {
                    Ok(_) => set_success_message.set(Some(true)),
                    Err(_) => set_success_message.set(Some(false)),
                }
            }
        },
        false,
    );

    view! {
        <div
            class="bg-neon-shade-1000 border-2 border-neon-shade-900 absolute top-full right-10 p-5 rounded-sm"
            style="margin-top: -2rem;"
            class:hidden=move || !show.get()
        >
            {move || match success_message.get() {
                Some(true) => {
                    view! {
                        <div class="w-80 h-44 flex justify-center items-center textarea-result">
                            "Thanks for the feedback!"
                        </div>
                        <div class="flex gap-3 pt-2 justify-end">
                            <Button
                                attr:disabled=true
                                variant=ButtonVariant::Tertiary
                                attr:r#type="button"
                                on_click=Box::new(move || {
                                    set_success_message.set(None);
                                    set_show.set(false);
                                })
                            >
                                "Cancel"
                            </Button>
                            <Button
                                variant=ButtonVariant::Primary
                                attr:r#type="button"
                                on_click=Box::new(move || {
                                    set_show.set(false);
                                    set_success_message.set(None);
                                })
                            >
                                "Thank you!"
                            </Button>
                        </div>
                    }
                        .into_any()
                }
                Some(false) => {
                    view! {
                        <div class="w-80 h-44 flex justify-center items-center textarea-result">
                            "There was an error submitting your feedback. Please try again."
                        </div>
                        <div class="flex gap-3 pt-2 justify-end">
                            <Button
                                attr:disabled=true
                                variant=ButtonVariant::Tertiary
                                attr:r#type="button"
                                on_click=Box::new(move || {
                                    set_success_message.set(None);
                                    set_show.set(false);
                                })
                            >
                                "Cancel"
                            </Button>
                            <Button
                                variant=ButtonVariant::Primary
                                attr:r#type="button"
                                on_click=Box::new(move || {
                                    set_show.set(false);
                                    set_success_message.set(None);
                                })
                            >
                                "Try again"
                            </Button>
                        </div>
                    }
                        .into_any()
                }
                _ => {
                    view! {
                        <ActionForm action=submit_feedback>
                            <div>
                                <textarea
                                    name="feedback"
                                    required
                                    placeholder="How can we improve?"
                                    class="w-80 h-44"
                                />
                            </div>
                            <div class="flex gap-3 pt-2 justify-end">
                                <Button
                                    variant=ButtonVariant::Tertiary
                                    attr:r#type="button"
                                    on_click=Box::new(move || {
                                        set_show.set(false);
                                        set_success_message.set(None);
                                    })
                                >
                                    "Cancel"
                                </Button>
                                <Button
                                    variant=ButtonVariant::Primary
                                    attr:r#type="submit"
                                    attr:disabled=Signal::derive(move || {
                                        submit_feedback.pending().get()
                                    })
                                >
                                    "Submit"
                                </Button>
                            </div>
                        </ActionForm>
                    }
                        .into_any()
                }
            }}

        </div>
    }
}

#[server(GetOrganizations, "/api")]
pub async fn get_organizations() -> Result<Vec<Organization>, ServerFnError> {
    use crate::auth::utils::get_logged_in_user;
    let user = get_logged_in_user()
        .await?
        .ok_or(ServerFnError::new("not authorized"))?;
    let state: crate::AppState =
        use_context().ok_or(ServerFnError::new("internal server error"))?;
    let organizations = Organization::get_with_user(user.id, &state.pool).await?;
    Ok(organizations)
}

#[component]
pub fn OrgDropdown() -> impl IntoView {
    let organizations = Resource::new(|| (), move |_| async move { get_organizations().await });
    let organization: String = match use_context::<Organization>() {
        Some(organization) => organization.name,
        None => "Organizations".to_string(),
    };

    view! {
        <Suspense fallback=move || {
            view! {}
        }>
            {move || {
                organizations
                    .get()
                    .map(|organizations| {
                        let org = organization.clone();
                        match organizations {
                            Ok(organizations) => {
                                if organizations.len() <= 1 {
                                    view! {}.into_any()
                                } else {
                                    view! {
                                        <Dropdown label=org>
                                            {organizations
                                                .iter()
                                                .map(|org| {
                                                    let org_id = org.id;
                                                    let org_name = org.name.clone();
                                                    view! {
                                                        <DropdownItem on_click=Box::new(move || {
                                                            let _ = window()
                                                                .location()
                                                                .set_pathname(&format!("/dashboard/{}/overview", org_id));
                                                        })>{org_name}" - "{org_id}</DropdownItem>
                                                    }
                                                })
                                                .collect_view()
                                                .into_any()}
                                        </Dropdown>
                                    }
                                        .into_any()
                                }
                            }
                            Err(_) => view! {}.into_any(),
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn top_nav(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    let (show_feedback, set_show_feedback) = signal(false);
    let user = use_context::<crate::user::User>();

    let logout = Action::new(|_: &()| async move { logout_user().await });

    let value = logout.value();

    let auth_link = match user {
        Some(user) => view! {
            <button
                class="btn nav"
                on:click=move |_| {
                    logout.dispatch(());
                }
            >
                {move || {
                    if value.get().map(|x| x.ok()).flatten().is_some() {
                        user.refresh()
                    }
                }}
                {"Log out"}
            </button>
            <Button
                variant=ButtonVariant::Secondary
                on_click=Box::new(move || set_show_feedback.set(!show_feedback.get()))
            >
                "Feedback"
            </Button>
            <Feedback show=show_feedback set_show=set_show_feedback />
        }
        .into_any(),
        None => view! {
            <a href="/sign-in" class="btn secondary">
                {"Log in"}
            </a>
        }
        .into_any(),
    };

    view! {
        <nav class="fixed top-0 left-0 w-full bg-neutral-900 z-40">
            <div class="flex justify-between w-full p-10 items-center">
                <a href="https://www.getsundry.app/" class="text-4xl font-ibm">
                    sundry_
                </a>
                <div class="relative flex w-full items-center justify-center flex-col-reverse xl:flex-row gap-5">
                    <div class="xl:absolute left-8">
                        <OrgDropdown />
                    </div>
                    {match children {
                        Some(children) => children(),
                        None => view! { <div></div> }.into_any(),
                    }}
                </div>
                <div class="flex flex-row gap-3 whitespace-nowrap">{auth_link}</div>
            </div>
        </nav>
    }
}
