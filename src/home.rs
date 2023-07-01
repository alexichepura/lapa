use leptos::*;

use crate::post_list::PostList;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <section>
            <p>
                "Welcome to LAPA - Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website"
            </p>
        </section>
        <hr/>
        <PostList/>
    }
}
