use leptos::*;
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
    let (is_routing, set_is_routing) = create_signal(false);

    let user_json = serde_json::to_string(&user).unwrap();
    let user_script = format!("window.USER = {user_json};");
    let user_signal = create_rw_signal(user.clone());
    provide_context(user_signal);

    let settings_json = serde_json::to_string(&settings).unwrap();
    let settings_script = format!("window.SETTINGS = {settings_json};");
    let settings_signal = create_rw_signal(settings);
    provide_context(settings_signal);

    let formatter = |text| format!("{text} - Admin");

    view! {
        <Stylesheet id="leptos" href="/pkg/lapa_admin.css"/>
        <Title formatter/>
        <Favicons/>
        <Script>{user_script}</Script>
        <Script>{settings_script}</Script>
        <RoutingProgress
            is_routing
            max_time=std::time::Duration::from_millis(250)
            class="RoutingProgress"
        />
        <Router set_is_routing>
            <AdminRoutes user_signal/>
        </Router>
    }
}

#[component]
pub fn AdminRoutes(user_signal: RwSignal<Option<User>>) -> impl IntoView {
    create_effect(move |prev: Option<Option<User>>| {
        let user = user_signal();
        // log!("AdminRoutes user {:?}", user);
        // log!("AdminRoutes prev {:?}", prev);
        if let Some(prev) = prev {
            if user.is_some() && prev.is_none() {
                let navigate = use_navigate();
                navigate(&"/", Default::default()).expect("home route");
            }
        }
        user
    });
    view! {
        <Routes>
            <ProtectedRoute path="/login" redirect_path="/" condition={move || user_signal.get().is_none()} view=Login/>
            <ProtectedRoute path="/*" redirect_path="/login" condition={move || user_signal.get().is_some()} view=LayoutWithUser>
                <Route path="/" view=HomePage/>
                <Route path="/posts" view=PostList/>
                <Route path="/posts/new" view=PostNew/>
                <Route path="/posts/:id" view=PostPage/>
                <Route path="/settings" view=Settings/>
            </ProtectedRoute>
        </Routes>
    }
}
#[component]
pub fn LayoutWithUser() -> impl IntoView {
    let user_signal = use_user_signal();
    let user = user_signal.get_untracked().unwrap(); // at this point user must be some
    view! {<Layout user/>}
}

#[component]
pub fn AdminRouter() -> impl IntoView {
    let user_signal = create_rw_signal(None);
    view! {
        <Router>
            <AdminRoutes user_signal/>
        </Router>
    }
}

#[component]
pub fn Favicons() -> impl IntoView {
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
        <Link rel="manifest" href="/site.webmanifest"/>
    }
}
