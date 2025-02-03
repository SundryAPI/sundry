use leptos::prelude::*;

#[derive(Default)]
pub enum LoadingWheelSize {
    #[default]
    Regular,
    Small,
    Large,
}

#[component]
pub fn LoadingWheel(
    #[prop(default = LoadingWheelSize::Regular)] size: LoadingWheelSize,
) -> impl IntoView {
    let size_class = match size {
        LoadingWheelSize::Small => "text-[24px]",
        LoadingWheelSize::Regular => "text-[48px]",
        LoadingWheelSize::Large => "text-[96px]",
    };

    view! {
        <div class=format!(
            "spinner {}",
            size_class,
        )>{(0..12).map(|_| view! { <div class="spinner-blade"></div> }).collect::<Vec<_>>()}</div>
    }
}

#[component]
pub fn LoadingWheelCover(
    #[prop(default = LoadingWheelSize::Regular)] size: LoadingWheelSize,
) -> impl IntoView {
    view! {
        <div class="overlay">
            <LoadingWheel size=size />
        </div>
    }
}
