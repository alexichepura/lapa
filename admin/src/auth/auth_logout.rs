use leptos::*;
use leptos_router::{use_navigate, ActionForm};

use crate::auth::use_user_signal;

#[component]
pub fn Logout() -> impl IntoView {
    let logout = create_server_action::<Logout>();
    let value = logout.value();
    let user_signal = use_user_signal();
    create_effect(move |_| {
        if let Some(_) = value() {
            user_signal.set(None);
            let navigate = use_navigate();
            navigate(&"/", Default::default()).expect("home route");
        }
    });
    view! {
        <ActionForm action=logout>
            <button type="submit">Log Out</button>
        </ActionForm>
    }
}

#[server(Logout, "/auth")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = crate::server::use_auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
