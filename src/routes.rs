use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::{home::HomePage, product_page::ProductPage};

#[component]
pub fn FrontRoutes() -> impl IntoView {
    view! {
        <Routes fallback=|| "Page not found">
            <Route
                path=StaticSegment("")
                view=|| {
                    view! { <HomePage /> }
                }
            />
            <Route
                path=(StaticSegment("post"), ParamSegment("slug"))
                view=|| {
                    view! { <ProductPage /> }
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
            <FrontRoutes />
        </Router>
    }
}
