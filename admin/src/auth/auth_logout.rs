use leptos::*;
use leptos_router::ActionForm;
use crate::auth::use_user_signal;

#[component]
pub fn Logout() -> impl IntoView {
    let logout = create_server_action::<Logout>();
    let value = logout.value();
    let user_signal = use_user_signal();
    create_effect(move |_| {
        if let Some(v) = value() {
            tracing::debug!("Logout effect: value: {:?}", &v);
            user_signal.set(None);
        }
    });
    let (is_skip_redirect, set_skip_redirect) = create_signal(false);
    create_effect(move |_| {
        request_animation_frame(move || {
            set_skip_redirect(true);
        });
    });
    let skip_redirect_view = move || match is_skip_redirect() {
        true => view! { <input type="hidden" name="skip_redirect" value="1"/> }.into_view(),
        false => ().into_view(),
    };
    view! {
        <ActionForm action=logout>
            {skip_redirect_view} <button type="submit">Log Out</button>
        </ActionForm>
    }
}

#[server(Logout, "/auth")]
pub async fn logout(skip_redirect: Option<String>) -> Result<(), ServerFnError> {
    let auth = crate::server::use_auth()?;
    auth.logout_user();
    if skip_redirect.is_none() {
        leptos_axum::redirect("/");
    }
    Ok(())
}
