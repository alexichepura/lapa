use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    auth::{Login, User},
    layout::Layout,
    settings::SettingsCx,
};

#[component]
pub fn App(cx: Scope, user: Option<User>, settings: SettingsCx) -> impl IntoView {
    provide_meta_context(cx);
    let (is_routing, set_is_routing) = create_signal(cx, false);

    let user_json = serde_json::to_string(&user).unwrap();
    let user_script = format!("window.USER = {user_json};");
    let user_signal = create_rw_signal(cx, user.clone());
    provide_context(cx, user_signal);

    let settings_json = serde_json::to_string(&settings).unwrap();
    let settings_script = format!("window.SETTINGS = {settings_json};");
    let settings_signal = create_rw_signal(cx, settings);
    provide_context(cx, settings_signal);

    let formatter = |text| format!("{text} - Admin");

    view! { cx,
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
            {move || match user_signal() {
                Some(user) => {
                    view! { cx, <Layout user/> }
                        .into_view(cx)
                }
                None => {
                    view! { cx,
                        <Login>
                            <span>"Logged out."</span>
                        </Login>
                    }
                        .into_view(cx)
                }
            }}
        </Router>
    }
}

#[component]
pub fn Favicons(cx: Scope) -> impl IntoView {
    view! { cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
        <Link rel="manifest" href="/site.webmanifest"/>
    }
}
