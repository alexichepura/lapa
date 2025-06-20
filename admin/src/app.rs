use components::{
    Outlet, ParentRoute, ProtectedParentRoute, ProtectedRoute, Route, Router, Routes,
    RoutingProgress,
};
use hooks::use_navigate;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    auth::{use_user_signal, Login, User}, category::{CategoryList, CategoryNew, CategoryPage}, home::HomePage, layout::Layout, post::{PostCreate, PostList, PostPage}, product::{ProductList, ProductNew, ProductPage}, settings::{Settings, SettingsCx}
};

#[component]
pub fn App(user: Option<User>, settings: SettingsCx) -> impl IntoView {
    provide_meta_context();
    let (is_routing, set_is_routing) = signal(false);

    let user_signal = RwSignal::new(user.clone());
    provide_context(user_signal);
    let settings_signal = RwSignal::new(settings);
    provide_context(settings_signal);

    let formatter = |text| format!("{text} - Admin");
    view! {
        <Title formatter />
        <RoutingProgress
            is_routing
            max_time=std::time::Duration::from_millis(250)
            class:RoutingProgress
        />
        <Router set_is_routing>
            <AdminRoutes user_signal />
        </Router>
    }
}

#[component]
pub fn AdminRoutes(user_signal: RwSignal<Option<User>>) -> impl IntoView {
    Effect::new(move |prev: Option<Option<User>>| {
        let user = user_signal();
        tracing::debug!("AdminRoutes effect: user: {:?}; prev: {:?}", &user, &prev);
        if let Some(prev) = prev {
            if user.is_some() && prev.is_none() {
                tracing::debug!("prev is none, user is some: {:?}", &user);
                let navigate = use_navigate();
                navigate(&"/", Default::default());
            }
            if user.is_none() && prev.is_some() {
                tracing::debug!("user is none, prev is some: {:?}", &prev);
                let navigate = use_navigate();
                navigate(&"/login", Default::default());
            }
        }
        user
    });
    view! {
        <Routes fallback=|| "Page not found">
            <ProtectedRoute
                path=StaticSegment("/login")
                redirect_path=|| "/"
                condition=move || Some(user_signal.get().is_none())
                view=Login
            />
            <ProtectedParentRoute
                path=StaticSegment("")
                redirect_path=|| "/login"
                condition=move || Some(user_signal.get().is_some())
                view=LayoutWithUser
            >
                <Route path=StaticSegment("/") view=HomePage />
                <Route path=StaticSegment("settings") view=Settings />
                <ParentRoute path=StaticSegment("product") view=|| view! { <Outlet /> }>
                    <Route path=StaticSegment("") view=ProductList />
                    <Route path=StaticSegment("new") view=ProductNew />
                    <Route path=ParamSegment("id") view=ProductPage />
                </ParentRoute>
                <ParentRoute path=StaticSegment("post-category") view=|| view! { <Outlet /> }>
                    <Route path=StaticSegment("") view=CategoryList />
                    <Route path=StaticSegment("new") view=CategoryNew />
                    <Route path=ParamSegment("id") view=CategoryPage />
                </ParentRoute>
                <ParentRoute path=StaticSegment("post") view=|| view! { <Outlet /> }>
                    <Route path=StaticSegment("") view=PostList />
                    <Route path=StaticSegment("create") view=PostCreate />
                    <Route path=ParamSegment("id") view=PostPage />
                </ParentRoute>
            // <ParentRoute path=StaticSegment("content") view=|| view! { <Outlet /> }>
            // <Route path=StaticSegment("") view=ContentList />
            // <Route path=StaticSegment("new") view=ContentNew />
            // <Route path=ParamSegment("id") view=ContentPage />
            // </ParentRoute>
            </ProtectedParentRoute>
        </Routes>
    }
}
#[component]
pub fn LayoutWithUser() -> impl IntoView {
    let user_signal = use_user_signal();
    let user = user_signal.get_untracked().unwrap(); // at this point user must be some
    view! { <Layout user /> }
}

#[component]
pub fn AdminRouter() -> impl IntoView {
    let user_signal = RwSignal::new(None);
    view! {
        <Router>
            <AdminRoutes user_signal />
        </Router>
    }
}
