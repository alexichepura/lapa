use leptos::{html::Nav, prelude::*};
use leptos_router::components::{Outlet, A};

use crate::auth::{Logout, User};

#[component]
pub fn Layout(user: User) -> impl IntoView {
    let el_blur: NodeRef<Nav> = NodeRef::new();

    let blur = move |_| {
        let _ = el_blur.get().expect("<nav> to exist").blur();
    };
    view! {
        <div class="admin">
            <header>
                <div class="logo">Admin</div>
                <span>{user.username}</span>
                <Logout />
            </header>
            <div class="menu" tabindex="0">
                <a class="toggle" href="#" tabindex="0">
                    Menu
                </a>
                <nav tabindex="1" on:click=blur node_ref=el_blur>
                    <A href="/" exact=true>
                        "Dashboard"
                    </A>
                    <A href="/product">Product</A>
                    <A href="/post-category">Post Category</A>
                    <A href="/post">Post</A>
                    <A href="/settings">Settings</A>
                </nav>
            </div>
            <main>
                <Outlet />
            </main>
        </div>
    }
}
