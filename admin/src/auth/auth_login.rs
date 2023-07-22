use super::auth_data::AuthError;
use crate::{
    auth::use_user_signal,
    form::{Checkbox, FormFooter, Input},
};
use leptos::*;
use leptos_router::{use_navigate, ActionForm};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginFormData {
    pub username: String,
    pub password: String,
}

#[component]
pub fn Login(cx: Scope, children: Children) -> impl IntoView {
    let login = create_server_action::<Login>(cx);
    let pending = login.pending();
    let value = login.value();
    let user_signal = use_user_signal(cx);

    let (is_skip_redirect, set_skip_redirect) = create_signal(cx, false);
    create_effect(cx, move |_| {
        request_animation_frame(move || {
            set_skip_redirect(true);
        });
    });
    let skip_redirect_view = move || match is_skip_redirect() {
        true => view! { cx, <input type="hidden" name="skip_redirect" value="1"/> }.into_view(cx),
        false => ().into_view(cx),
    };
    create_effect(cx, move |_| {
        if let Some(v) = value() {
            let login_result = v.map_err(|_| AuthError::ServerError).flatten();
            if let Ok(login_result) = login_result {
                user_signal.set(Some(login_result));
                let navigate = use_navigate(cx);
                navigate(&"/", Default::default()).expect("home route");
            }
        }
    });

    view! { cx,
        <fieldset disabled=move || pending() class="login-card">
            <legend>Log in</legend>
            <ActionForm action=login>
                {skip_redirect_view}
                <Input name="username" label="User"/>
                <Input name="password" label="Password" type_="password"/>
                <Checkbox name="remember" label="Remember me?"/>
                <FormFooter action=login submit_text="Login"/>
                <hr/>
                <div>{children(cx)}</div>
            </ActionForm>
        </fieldset>
    }
}

#[server(Login, "/auth")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
    remember: Option<String>,
    skip_redirect: Option<String>,
) -> Result<Result<super::User, AuthError>, ServerFnError> {
    let prisma_client = crate::prisma::use_prisma(cx)?;
    let user = prisma_client
        .user()
        .find_unique(prisma_client::db::user::username::equals(username))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(match user {
        None => {
            crate::server::serverr_401(cx);
            Err(AuthError::NoMatch)
        }
        Some(user) => {
            let auth = crate::server::use_server_auth(cx)?;
            match bcrypt::verify(password, &user.password)
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?
            {
                true => {
                    auth.login_user(user.id.clone());
                    auth.remember_user(remember.is_some());
                    if skip_redirect.is_none() {
                        leptos_axum::redirect(cx, "/");
                    }
                    Ok(super::User {
                        id: user.id,
                        username: user.username,
                    })
                }
                false => Err(AuthError::NoMatch),
            }
        }
    })
}
