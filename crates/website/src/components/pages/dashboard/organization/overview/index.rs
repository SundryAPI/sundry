use super::invite::InviteUser;
use super::sources::{ComingSoon, OrgSources, UserSources};
use super::user::UserApiKey;
use leptos::prelude::*;
use models::organization::{Organization, Role};

#[component]
pub fn Overview() -> impl IntoView {
    let user_role = use_context::<Role>().unwrap();
    let organization_id = use_context::<Organization>().unwrap().id;

    view! {
        <div class="flex flex-col gap-32">
            <div class="flex justify-between flex-col lg:flex-row gap-4">
                <h1 class="!text-9xl font-bold">"Welcome to Sundry"</h1>
                <div class="flex flex-col gap-4">
                    <div class="flex gap-2 p-4 items-start border-sm bg-neutral-800">
                        <span class="material-symbols-outlined text-neon-shade-100">check</span>
                        <p>"Connect a few data sources"</p>
                    </div>
                    <div class="flex gap-2 p-4 items-start border-sm bg-neutral-800">
                        <span class="material-symbols-outlined text-neon-shade-100">check</span>
                        <p>
                            "Test your integrations in the "
                            <a href=format!("/dashboard/{organization_id}/demo") rel="external">
                                "Sandbox"
                            </a>
                        </p>
                    </div>
                    <div class="flex gap-2 p-4 items-start border-sm bg-neutral-800">
                        <span class="material-symbols-outlined text-neon-shade-100">check</span>
                        <p>"Generate an API key to use it elsewhere"</p>
                    </div>
                </div>
            </div>

            <div class="text-center flex flex-col gap-9">
                <h3 class="flex gap-4 items-center">
                    <span class="material-symbols-outlined text-neon-tint-400 !text-5xl">
                        electric_bolt
                    </span>
                    "Connect your data source"
                </h3>
                {match user_role {
                    Role::Owner => view! { <OrgSources /> }.into_any(),
                    _ => view! { <UserSources /> }.into_any(),
                }}
            </div>

            <UserApiKey />

            {(user_role == Role::Owner).then(|| { view! { <InviteUser /> }.into_any() })}

            <div class="text-center flex flex-col gap-9">
                <h3 class="flex gap-2 mx-auto align-center">
                    "Coming soon "
                    <span class="material-symbols-outlined text-cornflower-tint-100 !text-5xl">
                        rocket_launch
                    </span>
                </h3>
                <ComingSoon />
            </div>
        </div>
    }
}
