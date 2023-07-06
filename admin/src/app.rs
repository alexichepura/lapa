use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    auth::{get_user, Login, User},
    layout::Layout,
    util::Loading,
};

#[component]
pub fn App(cx: Scope, user: Option<User>) -> impl IntoView {
    provide_meta_context(cx);
    let (is_routing, set_is_routing) = create_signal(cx, false);

    let formatter = |text| format!("{text} - Admin");
    view! { cx,
        <Stylesheet id="leptos" href="/pkg/lapa_admin.css"/>
        <Title formatter/>
        <Favicons/>
        <RoutingProgress
            is_routing
            max_time=std::time::Duration::from_millis(250)
            class="RoutingProgress"
        />
        <Router set_is_routing>
            {match user {
                Some(user) => view! { cx, <Layout user=user/> }.into_view(cx),
                None => {
                    view! { cx, <AsyncUserRoutes/> }
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

#[component]
pub fn AsyncUserRoutes(cx: Scope) -> impl IntoView {
    let user = create_blocking_resource(cx, || (), move |_| get_user(cx));

    view! { cx,
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || match user.read(cx) {
                None => view! { cx, <p>"User_None"</p> }.into_view(cx),
                Some(user) => {
                    match user {
                        Err(e) => {
                            view! { cx,
                                <Login>
                                    <span>{format!("Login error: {}", e)}</span>
                                </Login>
                            }
                                .into_view(cx)
                        }
                        Ok(None) => {
                            view! { cx,
                                <Login>
                                    <span>"Logged out."</span>
                                </Login>
                            }
                                .into_view(cx)
                        }
                        Ok(Some(user)) => view! { cx, <Layout user=user/> }.into_view(cx),
                    }
                }
            }}
        </Suspense>
    }
}
