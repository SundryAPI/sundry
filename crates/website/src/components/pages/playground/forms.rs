use crate::components::atoms::button::{Button, ButtonVariant};
use leptos::prelude::*;

#[server(TestAction, "/api")]
pub async fn test_action() -> Result<(), ServerFnError> {
    println!("test action");
    Ok(())
}

#[component]
pub fn Forms() -> impl IntoView {
    let action = ServerAction::<TestAction>::new();

    view! {
        <ActionForm action=action>
            <div class="auth">
                <div class="inputs">
                    <label class="auth">"Password" <input type="password" name="password" /></label>
                    <label class="auth">
                        "Repeat Password" <input type="password" name="repeat_password" />
                    </label>
                </div>
                <Button variant=ButtonVariant::Primary attr:r#type="submit">
                    Submit
                </Button>
            </div>
        </ActionForm>
    }
}
