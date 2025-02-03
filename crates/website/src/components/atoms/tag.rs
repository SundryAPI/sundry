use leptos::prelude::*;

pub struct Tag {
    icon: &'static str,
    classes: &'static str,
    text: &'static str,
}

pub enum TagVariant {
    Alpha,
    ComingSoon,
    Invited,
    Active,
}

#[component]
pub fn Tag(variant: TagVariant) -> impl IntoView {
    let tag = match variant {
        TagVariant::Alpha => Tag {
            icon: "electric_bolt",
            classes: "alpha",
            text: "Alpha",
        },
        TagVariant::ComingSoon => Tag {
            icon: "rocket_launch",
            classes: "coming-soon",
            text: "Coming Soon",
        },
        TagVariant::Invited => Tag {
            icon: "",
            classes: "invited",
            text: "Invited",
        },
        TagVariant::Active => Tag {
            icon: "",
            classes: "active",
            text: "Active",
        },
    };

    view! {
        <div class=format!(
            "tag {}",
            tag.classes,
        )>
            {if tag.icon.is_empty() {
                view! {}.into_any()
            } else {
                view! { <span class="material-symbols-outlined">{tag.icon}</span> }.into_any()
            }} {tag.text}
        </div>
    }
}

pub enum ActionTagVariant {
    Connect,
    Disconnect,
    Import,
    Remove,
}

pub enum ButtonAction {
    Action(Box<dyn Fn() + 'static>),
    Link(String),
}

#[component]
pub fn ActionTag(
    pending: ReadSignal<bool>,
    variant: ActionTagVariant,
    on_click: ButtonAction,
) -> impl IntoView {
    let tag = match variant {
        ActionTagVariant::Connect => Tag {
            icon: "login",
            classes: "connect",
            text: "Connect",
        },
        ActionTagVariant::Disconnect => Tag {
            icon: "logout",
            classes: "disconnect",
            text: "Disconnect",
        },
        ActionTagVariant::Import => Tag {
            icon: "add",
            classes: "import",
            text: "Import",
        },
        ActionTagVariant::Remove => Tag {
            icon: "remove",
            classes: "remove",
            text: "Remove",
        },
    };

    match on_click {
        ButtonAction::Action(on_click) => view! {
            <button
                class=format!("tag action-tag {}", tag.classes)
                on:click=move |_| { on_click() }
                disabled=move || pending.get()
            >
                {move || {
                    if pending.get() {
                        view! {
                            <span class="material-symbols-outlined">more_horiz</span>
                            {format!("{}{}", tag.text, "ing")}
                        }
                            .into_any()
                    } else {
                        view! {
                            <span class="material-symbols-outlined">{tag.icon}</span>
                            {tag.text}
                        }
                            .into_any()
                    }
                }}
            </button>
        }
        .into_any(),
        ButtonAction::Link(on_click) => view! {
            <a class=format!("tag action-tag {}", tag.classes) href=on_click>
                <span class="material-symbols-outlined">{tag.icon}</span>
                {tag.text}
            </a>
        }
        .into_any(),
    }
}
