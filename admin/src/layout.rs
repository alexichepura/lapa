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
            <div class="logo">"Admin"</div>
            <header>
                <div>
                    <button>
                        <A href="/posts/new">"Create post"</A>
                    </button>
                </div>
                <span>{user.username}</span>
                <Logout/>
            </header>
            <nav>
                <A href="/" exact=true>
                    "Dashboard"
                </A>
                <A href="/posts">"Posts"</A>
                <A href="/settings">"Settings"</A>
            </nav>
            <main>
                <AdminRoutes/>
            </main>
        </div>
    }
}
