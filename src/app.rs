use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{routes::FrontRoutes, settings::SettingsCx};

#[component]
pub fn App(settings: SettingsCx) -> impl IntoView {
    provide_meta_context();
    provide_context(settings.clone());
    let (is_routing, set_is_routing) = create_signal(false);

    let settings_json = serde_json::to_string(&settings).unwrap();
    let settings_script = format!("window.SETTINGS = {settings_json};");

    let formatter = |text| format!("{text} - LAPA");

    view! {
        <Html lang="en"/>
        <Stylesheet href="/css/open-props.min.css"/>
        <Stylesheet href="/css/normalize.min.css"/>
        <Stylesheet href="/css/buttons.min.css"/>
        <Stylesheet id="leptos" href="/pkg/lapa_site.css"/>
        <Title formatter/>
        <Favicons/>
        <Script>{settings_script}</Script>
        <RoutingProgress
            is_routing
            max_time=std::time::Duration::from_millis(250)
            class="RoutingProgress"
        />
        <Router set_is_routing>
            <header>
                <section>
                    <A href="/" exact=true>Home</A>
                    <a href="https://github.com/alexichepura/lapa">GitHub</a>
                </section>
            </header>
            <main>
                <FrontRoutes/>
            </main>
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
