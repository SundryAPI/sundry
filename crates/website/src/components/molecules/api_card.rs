use leptos::prelude::*;

#[derive(Copy, Clone)]
pub enum ApiCardVariant {
    Application,
    APIKey,
}

#[component]
pub fn ApiCard(
    #[prop(into)] title: String,
    description: impl IntoView + 'static,
    action: impl IntoView + 'static,
    variant: ApiCardVariant,
) -> impl IntoView {
    let title = StoredValue::new(title);
    view! {
        <div class="flex flex-row py-5 justify-between items-center">
            <div class="flex flex-col gap-2 text-left">
                <div>
                    {move || match variant {
                        ApiCardVariant::Application => {
                            view! { <h4>{title.read_value().to_string()}</h4> }.into_any()
                        }
                        ApiCardVariant::APIKey => {
                            view! { <h6>{title.read_value().to_string()}</h6> }.into_any()
                        }
                    }}
                </div>
                <div>{description}</div>
            </div>
            <div class="flex flex-col justify-between">{action}</div>
        </div>
    }
}
