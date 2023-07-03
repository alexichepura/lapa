use leptos::*;
use leptos_meta::Title;

use crate::post_list::PostList;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Home"/>
        <section>
            <h1>"Welcome"</h1>
            <p>
                "to LAPA - Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website"
            </p>
        </section>
        <hr/>
        <PostList/>
    }
}
