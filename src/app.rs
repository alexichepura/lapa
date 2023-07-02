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

    let settins_json = serde_json::to_string(&settings).unwrap();
    let settings_script = format!("var SETTINGS = {settins_json};");

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/lapa_site.css"/>
        <Title text="LAPA"/>
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
