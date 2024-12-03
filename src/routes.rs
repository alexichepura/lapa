use leptos::prelude::*;
use leptos_router::*;

use crate::{home::HomePage, post_page::PostPage};

#[component]
pub fn FrontRoutes() -> impl IntoView {
    view! {
        <Routes>
            <Route
                path=""
                view=|| {
                    view! { <HomePage/> }
                }
            />
            <Route
                path="/post/:slug"
                view=|| {
                    view! { <PostPage/> }
                }
            />
        </Routes>
    }
}

#[component]
pub fn GenerateRouteList() -> impl IntoView {
    leptos_meta::provide_meta_context();
    view! {
        <Router>
            <FrontRoutes/>
        </Router>
    }
}
