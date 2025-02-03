use leptos::leptos_dom::{
    helpers::{document, set_timeout_with_handle, window},
    log,
};
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn Copy(children: Children) -> impl IntoView {
    let (trigger_copy, set_trigger_copy) = signal(false);

    Effect::watch(
        move || trigger_copy.get(),
        move |copy, _, _| {
            if *copy {
                let text = document()
                    .get_element_by_id("copy")
                    .unwrap()
                    .text_content()
                    .unwrap();
                log!("text: {:?}", text);
                let _ = window().navigator().clipboard().write_text(&text);
                let _ = set_timeout_with_handle(
                    move || {
                        set_trigger_copy.set(false);
                    },
                    Duration::new(2, 0),
                );
            }
        },
        false,
    );

    view! {
        <div class="flex gap-4 bg-neutral-700 rounded-sm justify-between p-2 items-center">
            <div id="copy" style="overflow: hidden; word-wrap: break-word">
                {children()}
            </div>
            <div class="relative">
                <div
                    class="absolute bg-neutral-800 p-2"
                    style="right: 28px"
                    class:hidden=move || { !trigger_copy.get() }
                >
                    "Copied!"
                </div>
                <button on:click=move |_| { set_trigger_copy.set(true) }>
                    <span class="material-symbols-outlined text-neon-tint-100 hover:text-neon-tint-300 active:text-neon-tint-500">
                        content_copy
                    </span>
                </button>
            </div>
        </div>
    }
}
