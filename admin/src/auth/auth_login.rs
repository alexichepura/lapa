use super::auth_data::AuthError;
use crate::{
    auth::use_user_signal,
    form::{Checkbox, FormFooter, Input},
};
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginFormData {
    pub username: String,
    pub password: String,
}

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
        <fieldset prop:disabled=move || pending() class="login-card">
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
    let db = crate::server::db::use_db().await?;
    let user = clorinde::queries::user::user_get_auth_by_username()
        .bind(&db, &username).opt()
        .await
        .map_err(|e| lib::emsg(e, "user_get_auth_by_username"))?;
    let Some(user) = user else {
        crate::server::serverr_401();
        return Ok(Err(AuthError::NoMatch));
    };
    let auth = crate::server::use_auth()?;
    let res = match bcrypt::verify(password, &user.password)
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        true => {
            auth.login_user(user.id.clone());
            auth.remember_user(remember.is_some());
            if skip_redirect.is_none() {
                leptos_axum::redirect("/");
            }
            Ok(super::User {
                id: user.id,
                username,
            })
        }
        false => Err(AuthError::NoMatch),
    };
    Ok(res)
}
