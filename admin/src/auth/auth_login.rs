use super::auth_data::AuthError;
use crate::{
    auth::use_user_signal,
    form::{Checkbox, FormFooter, Input},
};
use leptos::{either::Either, prelude::*};

stylance::import_crate_style!(style, "src/auth.module.css");

#[component]
pub fn Login() -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let pending = login.pending();
    let value = login.value();
    let user_signal = use_user_signal();

    let (is_skip_redirect, set_skip_redirect) = signal(false);
    Effect::new(move |_| {
        request_animation_frame(move || {
            set_skip_redirect(true);
        });
    });
    let skip_redirect_view = move || match is_skip_redirect() {
        true => Either::Left(view! { <input type="hidden" name="skip_redirect" value="1" /> }),
        false => Either::Right(()),
    };
    Effect::new(move |_| {
        if let Some(v) = value.get() {
            let login_result = v.map_err(|_| AuthError::ServerError).flatten();
            if let Ok(login_result) = login_result {
                user_signal.set(Some(login_result));
            }
        }
    });

    view! {
        <fieldset prop:disabled=pending class=style::loginCard>
            <legend>Log in</legend>
            <ActionForm action=login>
                {skip_redirect_view} <Input name="username" label="User" />
                <Input name="password" label="Password" type_="password" />
                <Checkbox name="remember" label="Remember me?" />
                <FormFooter action=login submit_text="Login" />
            </ActionForm>
        </fieldset>
    }
}

#[server(Login, "/auth")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
    skip_redirect: Option<String>,
) -> Result<Result<super::User, AuthError>, ServerFnError> {
    let mut auth = crate::server::use_auth()?;
    let creds = crate::server::auth::Credentials {
        username: username.clone(),
        password,
    };

    let user = match auth.authenticate(creds).await {
        Ok(Some(user)) => user,
        // Bad password or a deactivated account: the same opaque error.
        Ok(None) => return Ok(Err(AuthError::NoMatch)),
        // Infrastructure failure: surface it as a 500 rather than a login
        // rejection.
        Err(e) => {
            tracing::error!(error = %e, "login backend error");
            return Err(ServerFnError::new(e.to_string()));
        }
    };

    auth.login(&user)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if remember.is_some() {
        // A long-lived session; without "Remember me?" the layer's default
        // browser-session cookie applies.
        auth.session
            .set_expiry(Some(tower_sessions::Expiry::OnInactivity(
                time::Duration::days(30),
            )));
    }

    if skip_redirect.is_none() {
        leptos_axum::redirect("/");
    }
    Ok(Ok(super::User::from(&user)))
}
