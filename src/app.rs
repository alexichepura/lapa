use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::FrontRoutes;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! { cx,
        <Stylesheet id="leptos" href="/pkg/lapa_site.css"/>
        <Title text="LAPA"/>
        <Router>
            <main>
                <FrontRoutes/>
            </main>
        </Router>
    }
}
