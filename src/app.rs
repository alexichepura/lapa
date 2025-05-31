use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Router, RoutingProgress, A};

use crate::{routes::FrontRoutes, settings::SettingsCx};

#[component]
pub fn App(settings: SettingsCx) -> impl IntoView {
    provide_meta_context();
    provide_context(settings.clone());
    let (is_routing, set_is_routing) = signal(false);
    let formatter = |text| format!("{text} - Lapa");
    view! {
        <Title formatter />
        <RoutingProgress
            is_routing
            max_time=std::time::Duration::from_millis(250)
            class:RoutingProgress
        />
        <Router set_is_routing>
            <header>
                <section>
                    <A href="/" exact=true>
                        Home
                    </A>
                    <a href="https://github.com/alexichepura/lapa">GitHub</a>
                </section>
            </header>
            <main>
                <FrontRoutes />
            </main>
        </Router>
    }
}
