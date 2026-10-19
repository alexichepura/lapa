use leptos::either::Either;
use leptos::prelude::*;

use super::{Permission, use_user_signal};

/// Shown when the signed-in user's role does not grant a page's permission.
#[component]
pub fn Forbidden() -> impl IntoView {
    view! {
        <section>
            <h1>"Forbidden"</h1>
            <p>"Your role does not allow access to this page."</p>
            <a href="/">"Back to dashboard"</a>
        </section>
    }
}

/// Renders `children` only when the signed-in user holds `permission`, and a
/// [`Forbidden`] page otherwise. UI convenience only: server functions enforce
/// the same permissions.
#[component]
pub fn RequirePermission(permission: Permission, children: Children) -> impl IntoView {
    let user_signal = use_user_signal();
    if user_signal
        .get_untracked()
        .is_some_and(|user| user.can(permission))
    {
        Either::Left(children())
    } else {
        Either::Right(view! { <Forbidden /> })
    }
}
