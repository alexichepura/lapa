// use leptos::prelude::*;

// #[component]
// pub fn Signup() -> impl IntoView {
//     let signup = ServerAction::<Signup>::new();
//     view! {
//         <ActionForm action=signup>
//             <h1>Sign Up</h1>
//             <label>
//                 <span>User ID:</span>
//                 <input type="text" placeholder="User ID" maxlength="32" name="username" />
//             </label>
//             <br />
//             <label>
//                 <span>Password:</span>
//                 <input type="password" placeholder="Password" name="password" />
//             </label>
//             <br />
//             <label>
//                 <span>Confirm Password:</span>
//                 <input type="password" placeholder="Password again" name="password_confirmation" />
//             </label>
//             <br />
//             <label>
//                 <span>Remember me?</span>
//                 <input type="checkbox" name="remember" class="auth-input" />
//             </label>
//             <br />
//             <button type="submit" class="button">
//                 Sign Up
//             </button>
//         </ActionForm>
//     }
// }

// #[server(Signup, "/auth")]
// pub async fn signup(
//     username: String,
//     password: String,
//     password_confirmation: String,
//     remember: Option<String>,
// ) -> Result<(), ServerFnError> {
//     let prisma_client = crate::server::use_prisma()?;
//     let auth = crate::server::use_auth()?;

//     if password != password_confirmation {
//         return Err(ServerFnError::new("Passwords did not match.".to_string()));
//     }

//     let password_hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

//     prisma_client
//         .clone()
//         .user()
//         .create(username.clone(), password_hashed, vec![])
//         .exec()
//         .await
//         .map_err(|e| lib::emsg(e, "User create"))?;

//     let user = prisma_client
//         .user()
//         .find_unique(prisma_client::db::user::username::equals(username))
//         .exec()
//         .await
//         .map_err(|e| lib::emsg(e, "Settings find"))?
//         .unwrap();

//     auth.login_user(user.id);
//     auth.remember_user(remember.is_some());

//     leptos_axum::redirect("/");

//     Ok(())
// }
