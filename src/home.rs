use leptos::*;
use leptos_meta::{Meta, Title};
use serde::{Deserialize, Serialize};

use crate::{
    post_list::PostList,
    util::{Loading, ParagraphsByMultiline},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomeData {
    pub home_text: String,
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let home = create_blocking_resource(cx, || (), move |_| get_home(cx));

    view! { cx,
        <Title text="Home"/>
        <Meta name="description" content="Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website"/>
        <h1>Welcome to LAPA</h1>
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                home.read(cx)
                    .map(|home| match home {
                        Err(e) => view! { cx, <p>{e.to_string()}</p> }.into_view(cx),
                        Ok(home) => view! { cx,
                            <section>
                                <ParagraphsByMultiline text=home.home_text/>
                            </section>
                        }.into_view(cx),
                    })
            }}
        </Suspense>
        <hr/>
        <PostList/>
    }
}

#[server(GetHome, "/api")]
pub async fn get_home(cx: Scope) -> Result<HomeData, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma(cx)?;
    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({ home_text }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?
        .unwrap();

    Ok(HomeData {
        home_text: settings.home_text,
    })
}
