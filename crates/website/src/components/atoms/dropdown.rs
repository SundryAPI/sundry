use leptos::html::Button;
use leptos::*;
use leptos_dom::helpers::window_event_listener;
use prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;

#[component]
pub fn Dropdown(
    #[prop(default = "Dropdown".to_string())] label: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = true)] close_on_select: bool,
    children: Children,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let toggle_dropdown = move |_| {
        if !disabled {
            set_is_open.update(|open| *open = !*open);
        }
    };

    // Close dropdown when clicking outside
    let dropdown_ref = NodeRef::<Button>::new();
    window_event_listener(ev::click, move |event| {
        if let Some(dropdown_el) = dropdown_ref.try_get().flatten() {
            let click_target = event.target();
            if let Some(target) = click_target {
                if !dropdown_el.contains(target.dyn_ref::<Node>()) {
                    set_is_open.set(false);
                }
            }
        }
    });

    let classes = if label.is_empty() {
        "dropdown-toggle m-auto"
    } else {
        "dropdown-toggle min-w-64 w-full"
    };

    view! {
        <div class="dropdown w-full">
            <button
                node_ref=dropdown_ref
                type="button"
                class=classes
                class:disabled=disabled
                class:show=move || is_open.get()
                on:click=move |e| {
                    toggle_dropdown(e);
                }
            >
                {if label.is_empty() {
                    view! {
                        <span class="material-symbols-outlined text-neon-tint-100 dropdown-arrow m-auto">
                            more_horiz
                        </span>
                    }
                        .into_any()
                } else {
                    view! {
                        {label}
                        <span class="material-symbols-outlined text-neon-tint-100 dropdown-arrow">
                            keyboard_arrow_down
                        </span>
                    }
                        .into_any()
                }}
            </button>
            <div
                class="dropdown-menu z-20 min-w-fit"
                class:show=move || is_open.get()
                on:click=move |e| {
                    if close_on_select {
                        toggle_dropdown(e);
                    }
                }
            >
                <ul class="mx-4 p-0">{children()}</ul>
            </div>
        </div>
    }
}

#[component]
pub fn DropdownItem(
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            class="dropdown-item w-full"
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </li>
    }
}
