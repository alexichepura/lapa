use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::{home::HomePage, post::post_page::PostPage, product_page::ProductPage};

#[component]
pub fn FrontRoutes() -> impl IntoView {
    view! {
        <Routes fallback=|| "Page not found">
            <Route path=StaticSegment("") view=HomePage />
            <Route path=(StaticSegment("post"), ParamSegment("slug")) view=ProductPage />
            <Route path=(ParamSegment("category_slug"), ParamSegment("slug")) view=PostPage />
        </Routes>
    }
}

#[component]
pub fn GenerateRouteList() -> impl IntoView {
    leptos_meta::provide_meta_context();
    view! {
        <Router>
            <FrontRoutes />
        </Router>
    }
}
