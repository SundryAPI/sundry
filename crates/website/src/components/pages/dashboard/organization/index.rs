use crate::components::atoms::slider::{Slider, SliderItemProp};
use crate::components::molecules::top_nav::TopNav;
use crate::components::pages::dashboard::organization::{
    demo::LoggedInDemo, developer::Developer, overview::index::Overview,
};
use leptos::leptos_dom::helpers::window;
use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use wasm_bindgen::JsValue;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Page {
    #[default]
    Overview,
    Demo,
    Developers,
}

use crate::components::molecules::footer::Footer;

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Overview => write!(f, "Overview"),
            Page::Demo => write!(f, "Demo"),
            Page::Developers => write!(f, "Developers"),
        }
    }
}

impl From<String> for Page {
    fn from(s: String) -> Self {
        match s.as_str() {
            "overview" => Page::Overview,
            "demo" => Page::Demo,
            "developers" => Page::Developers,
            _ => Page::Overview,
        }
    }
}

#[derive(Params, PartialEq)]
struct Tabs {
    tab: Option<String>,
}

#[component]
pub fn Index() -> impl IntoView {
    let tab = use_params()
        .read()
        .as_ref()
        .ok()
        .and_then(|params: &Tabs| params.tab.clone())
        .unwrap_or_else(move || "overview".to_string());

    let (page, set_page) = signal(Page::from(tab));

    Effect::new(move || {
        let path = window().location().pathname().expect("pathname");
        let mut path = path.split("/").collect::<Vec<&str>>();
        path.truncate(path.len() - 1);
        let path = path.join("/");
        let new = match page.get() {
            Page::Overview => "/overview",
            Page::Demo => "/demo",
            Page::Developers => "/developers",
        };
        let _ = window().history().expect("history").replace_state_with_url(
            &JsValue::from_str(""),
            "",
            Some(&format!("{}{}", path, new)),
        );
    });

    view! {
        <TopNav>
            {move || {
                view! {
                    <Slider items=vec![
                        SliderItemProp {
                            action: Box::new(move || set_page.set(Page::Overview)),
                            title: "Overview",
                            active: page.get() == Page::Overview,
                        },
                        SliderItemProp {
                            action: Box::new(move || set_page.set(Page::Demo)),
                            title: "Demo",
                            active: page.get() == Page::Demo,
                        },
                        SliderItemProp {
                            action: Box::new(move || set_page.set(Page::Developers)),
                            title: "Developers",
                            active: page.get() == Page::Developers,
                        },
                    ] />
                }
            }}

        </TopNav>
        {move || {
            match page.get() {
                Page::Overview => {
                    view! {
                        <DefaultLayout>
                            <Overview />
                        </DefaultLayout>
                    }
                        .into_any()
                }
                Page::Demo => {
                    view! {
                        <FullPageLayout>
                            <LoggedInDemo />
                        </FullPageLayout>
                    }
                        .into_any()
                }
                Page::Developers => {
                    view! {
                        <DefaultLayout>
                            <Developer />
                        </DefaultLayout>
                    }
                        .into_any()
                }
            }
        }}
    }
}

#[component]
fn DefaultLayout(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="px-10 pt-nav">
                <div class="w-full max-w-screen-xl m-auto min-h-with-nav py-20">{children()}</div>
            </div>
            <Footer />
        </div>
    }
}

#[component]
fn FullPageLayout(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="w-full max-w-screen-xl m-auto top-nav-padding">{children()}</div>
        </div>
    }
}
