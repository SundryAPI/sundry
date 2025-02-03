use crate::components::atoms::dropdown::{Dropdown, DropdownItem};
use leptos::*;
use logging::log;

#[component]
pub fn DropdownComp() -> impl IntoView {
    view! {
        <Dropdown label="Select Option".to_string()>
            <DropdownItem on_click=Box::new(|| log!("Option 1 clicked"))>"Option 1"</DropdownItem>
            <DropdownItem on_click=Box::new(|| log!("Option 2 clicked"))>"Option 2"</DropdownItem>
            <DropdownItem on_click=Box::new(|| log!("Option 3 clicked"))>"Option 3"</DropdownItem>
        </Dropdown>
    }
}
