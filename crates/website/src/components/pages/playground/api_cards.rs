use crate::components::atoms::tag::{ActionTag, ActionTagVariant, ButtonAction};
use crate::components::molecules::api_card::{ApiCard, ApiCardVariant};
use leptos::prelude::*;

#[server(ServerTest)]
pub async fn server_test() -> Result<(), ServerFnError> {
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(2));
    Ok(())
}

#[component]
pub fn APICards() -> impl IntoView {
    let (pending, set_pending) = signal(false);

    let _resource = Resource::new(
        move || pending.get(),
        move |pending| async move {
            if pending {
                let out = server_test().await;
                set_pending.set(false);
                out
            } else {
                Ok(())
            }
        },
    );

    view! {
        <ApiCard
            variant=ApiCardVariant::APIKey
            title="Title"
            description="Description"
            action=view! {
                <ActionTag
                    pending
                    variant=ActionTagVariant::Disconnect
                    on_click=ButtonAction::Action(Box::new(move || set_pending.set(true)))
                />
            }
        />
    }
}
