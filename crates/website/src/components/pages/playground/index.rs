use crate::components::atoms::button::{Button, ButtonVariant};
use crate::components::atoms::loading::LoadingWheelCover;
use crate::components::molecules::modal::Modal;
use crate::components::molecules::top_nav::TopNav;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::atoms::copy::Copy;

#[component]
pub fn ModalDemo() -> impl IntoView {
    let (show, set_show) = signal(false);
    let (show2, set_show2) = signal(false);
    let (show3, set_show3) = signal(false);

    let callback = move || {
        use_navigate()("/playground", Default::default());
    };

    view! {
        <div class="flex gap-4">
            <Button on_click=Box::new(move || {
                set_show.set(true)
            })>"Open Modal With Callback"</Button>
            <Modal show=(show, set_show) show_close_button=true on_close=Box::new(callback)>
                <div class="p-4">
                    <p>"This is a modal that will redirect after close"</p>
                </div>
            </Modal>

            <Button on_click=Box::new(move || {
                set_show2.set(true)
            })>"Open Modal No Callback"</Button>
            <Modal show=(show2, set_show2) show_close_button=true>
                <div class="p-4">
                    <p>"This is a modal"</p>
                </div>
            </Modal>

            <Button on_click=Box::new(move || {
                set_show3.set(true)
            })>"Open Modal No Close Button"</Button>
            <Modal show=(show3, set_show3) show_close_button=false>
                <div class="flex flex-col gap-4">
                    <div class="text-left">
                        <h6>"This access key is visible only once"</h6>
                        <p>
                            "Be sure to copy it to your clipboard and store it securely for future use."
                        </p>
                    </div>
                    <label>
                        "API Key" <Copy>"jhvkjhgbLJNGNKUGK&TYGBOYGNLjhkjbakdfkjhgahd"</Copy>
                    </label>
                </div>
            </Modal>
        </div>
    }
}

#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <div class="flex justify-center pt-4">
            <LoadingWheelCover />
        </div>
    }
}

#[component]
pub fn CopyComponent() -> impl IntoView {
    view! {
        <div style="width: 20rem">
            <Copy>
                "New content that is very long and shoudl be copied a;djf;akjdf ;lajd;fl kaj;df lkja;lkfj ;alskdjf ;akljdf ;aklj "
            </Copy>
        </div>
    }
}

#[component]
pub fn Buttons() -> impl IntoView {
    view! {
        <div class="flex gap-4 p-4">
            <Button attr:r#type="submit">"Primary Button"</Button>
            <Button variant=ButtonVariant::Secondary>"Secondary Button"</Button>
            <Button variant=ButtonVariant::Tertiary>"Tertiary Button"</Button>
            <Button variant=ButtonVariant::TertiaryWithIcon>
                <span class="material-symbols-outlined">arrow_back</span>
                <p>"Tertiary Button"</p>
            </Button>
        </div>
        <div class="flex gap-4 p-4">
            <Button attr:disabled=true>"Primary Button"</Button>
            <Button variant=ButtonVariant::Secondary attr:disabled=true>
                "Secondary Button"
            </Button>
            <Button variant=ButtonVariant::Tertiary attr:disabled=true>
                "Tertiary Button"
            </Button>
            <Button variant=ButtonVariant::TertiaryWithIcon attr:disabled=true>
                <span class="material-symbols-outlined">arrow_back</span>
                <p>"Tertiary Button"</p>
            </Button>
        </div>
    }
}

#[component]
pub fn TopNavView() -> impl IntoView {
    view! {
        <TopNav>
            <div class="flex flex-row gap-3">
                <div>Link 1</div>
                <div>Link 2</div>
                <div>Link 3</div>
            </div>
        </TopNav>
    }
}
