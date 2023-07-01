use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let logout = create_server_action::<Logout>(cx);
    view! { cx,
        <ActionForm action=logout>
            <button type="submit">"Log Out"</button>
        </ActionForm>
    }
}

#[server(Logout, "/auth")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let auth = super::ssr_auth(cx)?;

    auth.logout_user();
    leptos_axum::redirect(cx, "/");

    Ok(())
}
