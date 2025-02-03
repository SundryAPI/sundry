use super::index::{Buttons, CopyComponent, LoadingPage, ModalDemo, TopNavView};
use super::{
    api_cards::APICards, dropdown::DropdownComp, forms::Forms, source_cards::SourceCards,
    table::TablePlayground, tags::Tags,
};
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route};
use leptos_router::{path, MatchNestedRoutes};

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="p-4 flex flex-row">
            <div class="flex flex-col gap-2">
                <a href="/playground/buttons">"Buttons"</a>
                <a href="/playground/forms">"Forms"</a>
                <a href="/playground/top_nav">"Top Nav"</a>
                <a href="/playground/source_cards">"Source Cards"</a>
                <a href="/playground/loading">"Loading"</a>
                <a href="/playground/modal">"Modal"</a>
                <a href="/playground/tags">"Tags"</a>
                <a href="/playground/copy">"Copy"</a>
                <a href="/playground/api_cards">"API Cards"</a>
                <a href="/playground/dropdown">"Dropdown"</a>
                <a href="/playground/table">"Table"</a>
            </div>
            <div class="p-4 w-full">
                <Outlet />
            </div>
        </div>
    }
}

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/playground") view=Layout>
            <Route path=path!("/buttons") view=Buttons />
            <Route path=path!("/forms") view=Forms />
            <Route path=path!("/top_nav") view=TopNavView />
            <Route path=path!("/source_cards") view=SourceCards />
            <Route path=path!("/loading") view=LoadingPage />
            <Route path=path!("/modal") view=ModalDemo />
            <Route path=path!("/tags") view=Tags />
            <Route path=path!("/copy") view=CopyComponent />
            <Route path=path!("/api_cards") view=APICards />
            <Route path=path!("/dropdown") view=DropdownComp />
            <Route path=path!("/table") view=TablePlayground />
            <Route path=path!("") view=|| view! { <div>Choose a component</div> } />
        </ParentRoute>
    }
    .into_inner()
}
