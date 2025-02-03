use crate::components::atoms::button::{Button, ButtonVariant};
use crate::components::atoms::tag::{ActionTag, ActionTagVariant, ButtonAction};
use leptos::prelude::*;

pub enum SourceType {
    Connect,
    Disconnect,
}

pub struct Action {
    pub variant: SourceType,
    pub btn_action: ButtonAction,
}

#[component]
pub fn SourceCard(
    icon: impl IntoView + 'static,
    title: impl IntoView + 'static,
    description: impl IntoView + 'static,
    #[prop(optional)] action: Option<Action>,
) -> impl IntoView {
    let out = if let Some(action) = action {
        view! {
            <SourceCardWithAction
                content=view! { <CardContent icon=icon title=title description=description /> }
                action_t=action
            />
        }
        .into_any()
    } else {
        view! { <SourceCardNoAction icon=icon title=title description=description /> }.into_any()
    };

    view! { {out} }
}

#[component]
fn SourceCardNoAction(
    icon: impl IntoView,
    title: impl IntoView,
    description: impl IntoView,
) -> impl IntoView {
    view! {
        <div class="flex flex-row w-96 gap-4 p-5 bg-neutral-800">
            <CardContent icon=icon title=title description=description />
        </div>
    }
}

#[component]
fn CardContent(
    icon: impl IntoView,
    title: impl IntoView,
    description: impl IntoView,
) -> impl IntoView {
    view! {
        <div class="flex flex-row justify-between gap-4 text-left">
            <div class="h-full flex-none">{icon}</div>
            <div class="flex flex-col shrink">
                <h6>{title}</h6>
                <div>{description}</div>
            </div>
        </div>
    }
}

#[component]
fn SourceCardWithAction<T: IntoView>(content: T, action_t: Action) -> impl IntoView {
    let variant = match action_t {
        Action {
            variant: SourceType::Connect,
            ..
        } => ActionTagVariant::Connect,
        _ => ActionTagVariant::Disconnect,
    };

    let btn = match action_t.btn_action {
        ButtonAction::Action(action) => view! {
            <ActionTag
                variant=variant
                on_click=ButtonAction::Action(action)
                pending=signal(false).0
            />
        }
        .into_any(),
        ButtonAction::Link(link) => view! { <ActionTag variant=variant on_click=ButtonAction::Link(link) pending=signal(false).0 /> }
        .into_any(),
    };

    view! {
        <div class="flex flex-row w-full justify-between">
            {content} <div class="my-auto">{btn}</div>
        </div>
    }
}
