use super::auth_data::AuthError;
use crate::form::{Checkbox, FormFooter, Input};
use leptos::*;
use leptos_router::ActionForm;
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

    view! { cx,
        <fieldset disabled=move || pending() class="login-card">
            <legend>"Log in"</legend>
            <ActionForm action=login>
                <Input name="username" label="User"/>
                <Input name="password" label="Password" type_="password"/>
                <Checkbox name="remember" label="Remember me?"/>
                <FormFooter action=login/>
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
) -> Result<Result<(), AuthError>, ServerFnError> {
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
            crate::err::serverr_401(cx);
            Err(AuthError::NoMatch)
        }
        Some(user) => {
            let auth = super::ssr_auth(cx)?;
            match bcrypt::verify(password, &user.password)
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?
            {
                true => {
                    auth.login_user(user.id);
                    auth.remember_user(remember.is_some());
                    leptos_axum::redirect(cx, "/");
                    Ok(())
                }
                false => Err(AuthError::NoMatch),
            }
        }
    })
}
