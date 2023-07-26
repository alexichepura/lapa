use leptos::*;
use leptos_router::*;

use crate::{
    home::HomePage,
    post::{PostList, PostNew, PostPage},
    settings::Settings,
};

#[component]
pub fn AdminRoutes() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view=HomePage/>
            <Route path="/posts" view=PostList/>
            <Route path="/posts/new" view=PostNew/>
            <Route path="/posts/:id" view=PostPage/>
            <Route path="/settings" view=Settings/>
        </Routes>
    }
}

#[component]
pub fn GenerateRouteList() -> impl IntoView {
    leptos_meta::provide_meta_context();
    view! {
        <Router>
            <AdminRoutes/>
        </Router>
    }
}
