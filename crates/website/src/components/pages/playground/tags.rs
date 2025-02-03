use crate::components::atoms::tag::{ActionTag, ActionTagVariant, ButtonAction, Tag, TagVariant};
use leptos::prelude::*;

#[server(ServerTest)]
pub async fn server_test() -> Result<(), ServerFnError> {
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(2));
    Ok(())
}

#[component]
pub fn Tags() -> impl IntoView {
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
        <div class="flex gap-4 pb-4">
            <Tag variant=TagVariant::Alpha />
            <Tag variant=TagVariant::ComingSoon />
            <Tag variant=TagVariant::Active />
            <Tag variant=TagVariant::Invited />
        </div>
        <div class="flex gap-4">
            <ActionTag
                pending
                variant=ActionTagVariant::Connect
                on_click=ButtonAction::Action(Box::new(move || set_pending.set(true)))
            />

            <ActionTag
                pending
                variant=ActionTagVariant::Disconnect
                on_click=ButtonAction::Action(Box::new(move || set_pending.set(true)))
            />

            <ActionTag
                pending
                variant=ActionTagVariant::Import
                on_click=ButtonAction::Action(Box::new(move || set_pending.set(true)))
            />

            <ActionTag
                pending
                variant=ActionTagVariant::Remove
                on_click=ButtonAction::Action(Box::new(move || set_pending.set(true)))
            />
        </div>
    }
}
