use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(into)] show: (ReadSignal<bool>, WriteSignal<bool>),
    #[prop(default = false)] show_close_button: bool,
    #[prop(optional)] on_close: Option<Box<dyn Fn() + Send + Sync + 'static>>,
    #[prop(optional, into)] allow_close: Option<Signal<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let (close_pressed, set_close_pressed) = signal(false);
    let (show, set_show) = show;

    Effect::watch(
        move || close_pressed.get(),
        move |close_pressed, _, _| {
            if *close_pressed {
                let close = if let Some(allow_close) = allow_close {
                    allow_close.get()
                } else {
                    true
                };

                if close {
                    set_show.set(false);
                    if let Some(callback) = &on_close {
                        callback();
                    }
                }
            }
        },
        false,
    );

    let show_close_button = move || {
        show_close_button
            && allow_close
                .map(|allow_close| allow_close.get())
                .unwrap_or(true)
    };

    view! {
        {move || {
            show.get()
                .then(|| {
                    view! {
                        <div
                            class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
                            on:click=move |_| set_close_pressed.set(true)
                        >
                            <div
                                class="bg-neutral-800 rounded-sm p-10 max-w-lg w-full mx-4 relative border-neon-shade-900 border-2"
                                on:click=|e| e.stop_propagation()
                            >
                                {move || {
                                    show_close_button()
                                        .then(|| {
                                            view! {
                                                <button
                                                    class="absolute top-2 right-2 text-gray-500 hover:text-gray-700"
                                                    on:click=move |e| {
                                                        e.stop_propagation();
                                                        set_close_pressed.set(true)
                                                    }
                                                >
                                                    "X"
                                                </button>
                                            }
                                        })
                                }}
                                {children()}
                            </div>
                        </div>
                    }
                })
        }}
    }
}
