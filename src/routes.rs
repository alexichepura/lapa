use leptos::*;
use leptos_router::*;

use crate::{home::HomePage, post_page::PostPage};

#[component]
pub fn FrontRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Routes>
            <Route
                path=""
                view=|cx| {
                    view! { cx, <HomePage/> }
                }
            />
            <Route
                path="/post/:slug"
                view=|cx| {
                    view! { cx, <PostPage/> }
                }
            />
        </Routes>
    }
}

#[component]
pub fn GenerateRouteList(cx: Scope) -> impl IntoView {
    leptos_meta::provide_meta_context(cx);
    view! { cx,
        <Router>
            <FrontRoutes/>
        </Router>
    }
}
