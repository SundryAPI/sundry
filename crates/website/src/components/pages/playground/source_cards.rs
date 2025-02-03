use crate::components::atoms::tag::ButtonAction;
use crate::components::molecules::source_card::{Action, SourceCard, SourceType};
use leptos::prelude::*;

#[component]
pub fn SourceCards() -> impl IntoView {
    view! {
        <h2>"With Action"</h2>
        <div class="flex flex-col gap-5 max-w-screen-md m-auto">
            <SourceCard
                icon=view! { <img src="/images/gmail.svg" /> }
                title=view! { "Gmail" }
                description=view! {
                    "Connect your Gmail account and have access to all your email data."
                }
                action=Action {
                    variant: SourceType::Connect,
                    btn_action: ButtonAction::Link(String::from("")),
                }
            />
            <SourceCard
                icon=view! { <img src="/images/discord.svg" /> }
                title=view! { "Discord" }
                description=view! {
                    "Connect your Discord account and access all your conversations."
                }
                action=Action {
                    variant: SourceType::Disconnect,
                    btn_action: ButtonAction::Link(String::from("")),
                }
            />
        </div>
        <h2>"Without Action"</h2>
        <div class="flex flex-col">
            <div class="flex flex-row gap-2 justify-center flex-wrap">
                <SourceCard
                    icon=view! { <img src="/images/discord.svg" /> }
                    title=view! { "Discord" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/confluence.svg" /> }
                    title=view! { "Confluence" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/google_drive.svg" /> }
                    title=view! { "Google Drive" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/gmail.svg" /> }
                    title=view! { "Gmail" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/guru.svg" /> }
                    title=view! { "Guru" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/jira.svg" /> }
                    title=view! { "Jira" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/salesforce.svg" /> }
                    title=view! { "Salesforce" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/sharepoint.svg" /> }
                    title=view! { "Sharepoint" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/slab.svg" /> }
                    title=view! { "Slab" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/stack_overflow.svg" /> }
                    title=view! { "Stack Overflow" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/zendesk.svg" /> }
                    title=view! { "Zendesk" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/teams.svg" /> }
                    title=view! { "Teams" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/youtube.svg" /> }
                    title=view! { "Youtube" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/notion.svg" /> }
                    title=view! { "Notion" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
                <SourceCard
                    icon=view! { <img src="/images/question_answer.svg" /> }
                    title=view! { "Can't find what you're looking for?" }
                    description=view! {
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accu santium dolor emque lauda ntium, totam rem aperiam."
                    }
                />
            </div>
        </div>
    }
}
