use leptos::*;
use leptos_router::{use_navigate, ActionForm};

use crate::auth::use_user_signal;

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let logout = create_server_action::<Logout>(cx);
    let value = logout.value();
    let user_signal = use_user_signal(cx);
    create_effect(cx, move |_| {
        if let Some(_) = value() {
            user_signal.set(None);
            let navigate = use_navigate(cx);
            navigate(&"/", Default::default()).expect("home route");
        }
    });
    view! { cx,
        <ActionForm action=logout>
            <button type="submit">Log Out</button>
        </ActionForm>
    }
}

#[server(Logout, "/auth")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let auth = crate::server::use_server_auth(cx)?;

    auth.logout_user();
    leptos_axum::redirect(cx, "/");

    Ok(())
}
