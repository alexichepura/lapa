use components::{
    Outlet, ParentRoute, ProtectedParentRoute, ProtectedRoute, Route, Router, Routes,
    RoutingProgress,
};
use hooks::use_navigate;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    auth::{use_user_signal, Login, User},
    home::HomePage,
    layout::Layout,
    post::{PostList, PostNew, PostPage},
    settings::{Settings, SettingsCx},
};

#[component]
pub fn App(user: Option<User>, settings: SettingsCx) -> impl IntoView {
    provide_meta_context();
    let (is_routing, set_is_routing) = signal(false);

    let user_json = serde_json::to_string(&user).unwrap();
    let user_script = format!("window.USER = {user_json};");
    let user_signal = RwSignal::new(user.clone());
    provide_context(user_signal);

    let settings_json = serde_json::to_string(&settings).unwrap();
    let settings_script = format!("window.SETTINGS = {settings_json};");
    let settings_signal = RwSignal::new(settings);
    provide_context(settings_signal);

    let formatter = |text| format!("{text} - Admin");

    view! {
        <Stylesheet id="leptos" href="/pkg/admin.css" />
        <Title formatter />
        <Favicons />
        <Script>{user_script}</Script>
        <Script>{settings_script}</Script>
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
                <Route path=StaticSegment("posts") view=PostList />
                <ParentRoute path=StaticSegment("posts") view=|| view! { <Outlet /> }>
                    <Route path=StaticSegment("new") view=PostNew />
                    <Route path=ParamSegment("id") view=PostPage />
                </ParentRoute>
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

#[component]
pub fn Favicons() -> impl IntoView {
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <Link rel="manifest" href="/site.webmanifest" />
    }
}
