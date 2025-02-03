use leptos::prelude::*;

pub struct SliderItemProp {
    pub action: Box<dyn Fn() -> ()>,
    pub title: &'static str,
    pub active: bool,
}

#[component]
fn SliderItem(
    action: Box<dyn Fn() -> ()>,
    title: &'static str,
    active_signal: (ReadSignal<i32>, WriteSignal<i32>),
    child_index: i32,
) -> impl IntoView {
    view! {
        <button
            class="slider-item"
            class:active=move || active_signal.0.get() == child_index
            on:click=move |_| {
                active_signal.1.set(child_index);
                action();
            }
        >
            {title}
        </button>
    }
}

#[component]
pub fn Slider(items: Vec<SliderItemProp>) -> impl IntoView {
    let (active, set_active) = signal(0);

    let children = items
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            view! {
                {move || {
                    if x.active {
                        set_active.set(i as i32);
                    }
                }}
                <SliderItem
                    action=x.action
                    title=x.title
                    active_signal=(active.into(), set_active.clone())
                    child_index=i as i32
                />
            }
            .into_any()
        })
        .collect_view();

    view! { <div class="slider flex flex-row gap-3">{children}</div> }
}
