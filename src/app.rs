use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::routes::FrontRoutes;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct SettingsCx {
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

#[component]
pub fn App(cx: Scope, settings: SettingsCx) -> impl IntoView {
    provide_meta_context(cx);
    provide_context(cx, settings);

    let settings_json = serde_json::to_string(&settings).unwrap();
    let settings_script = format!("var SETTINGS = {settings_json};");

    let formatter = |text| format!("{text} - LAPA");

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/lapa_site.css"/>
        <Title formatter/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
        <Link rel="manifest" href="/site.webmanifest"/>
        <Script>
            "console.log('Meta rendered twice!!!');"
            {settings_script}
        </Script>
        <Router>
            <main>
                <FrontRoutes/>
            </main>
        </Router>
    }
}

#[component]
pub fn GenerateRouteList(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <FrontRoutes/>
        </Router>
    }
}
