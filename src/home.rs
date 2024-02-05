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
pub fn HomePage() -> impl IntoView {
    let home = create_blocking_resource(|| (), move |_| get_home());

    view! {
        <Title text="Home"/>
        <Meta
            name="description"
            content="Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website"
        />
        <h1>Welcome to LAPA</h1>
        <Suspense fallback=move || {
            view! { <Loading/> }
        }>
            {move || {
                home.get()
                    .map(|home| match home {
                        Err(e) => view! { <p>{e.to_string()}</p> }.into_view(),
                        Ok(home) => {
                            view! {
                                <section>
                                    <ParagraphsByMultiline text=home.home_text/>
                                </section>
                            }
                                .into_view()
                        }
                    })
            }}

        </Suspense>
        <hr/>
        <PostList/>
    }
}

#[server(GetHome, "/api")]
pub async fn get_home() -> Result<HomeData, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;
    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({ home_text }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::new("Server error".to_string())
        })?
        .unwrap();

    Ok(HomeData {
        home_text: settings.home_text,
    })
}
