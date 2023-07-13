use leptos::*;
use leptos_router::A;

use crate::{
    auth::{Logout, User},
    routes::AdminRoutes,
};

#[component]
pub fn Layout(cx: Scope, user: User) -> impl IntoView {
    view! { cx,
        <div class="admin">
            <header>
                <div class="logo">"Admin"</div>
                <span>{user.username}</span>
                <Logout/>
            </header>
            <div class="menu">
                <a class="toggle" href="#">"Menu"</a>
                <nav>
                    <A href="/" exact=true>
                        "Dashboard"
                    </A>
                    <A href="/posts">"Posts"</A>
                    <A href="/settings">"Settings"</A>
                </nav>
            </div>
            <main>
                <AdminRoutes/>
            </main>
        </div>
    }
}
