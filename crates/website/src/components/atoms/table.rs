use leptos::html::Button;
use leptos::*;
use leptos_dom::helpers::window_event_listener;
use logging::log;
use prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;

#[component]
pub fn Table(children: ChildrenFragment) -> impl IntoView {
    let children = children().nodes;
    let mut children = children.into_iter();
    let header = children.next().unwrap();
    let rows = children;

    view! {
        <table class="table-auto w-full">
            <thead>{header}</thead>
            <tbody>
                {rows
                    .map(|row| {
                        view! { {row} }
                    })
                    .collect_view()
                    .into_any()}
            </tbody>
        </table>
    }
}

pub enum RowType {
    Row,
    Header,
}

#[component]
pub fn Row(
    #[prop(default=RowType::Row)] _type: RowType,
    children: ChildrenFragment,
) -> impl IntoView {
    let class = match _type {
        RowType::Row => "",
        RowType::Header => "text-neon-shade-100 font-semibold",
    };

    view! {
        <tr class=format!(
            r#"{class}"#,
        )>
            {children()
                .nodes
                .into_iter()
                .map(|child| {
                    view! { <td>{child}</td> }
                })
                .collect_view()
                .into_any()}
        </tr>
    }
}
