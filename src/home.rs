use leptos::{either::EitherOf3, prelude::*};
use leptos_meta::{Meta, Title};
use serde::{Deserialize, Serialize};

use crate::{
    err::AppError,
    post_list::PostList,
    util::{AlertDanger, ParagraphsByMultiline},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomeData {
    pub home_text: String,
}

#[component]
pub fn HomePage() -> impl IntoView {
    let home = Resource::new_blocking(|| (), move |_| get_home());

    view! {
        <Title text="Home" />
        <Meta
            name="description"
            content="Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website"
        />
        <h1>Welcome to LAPA</h1>
        <Suspense>
            {move || Suspend::new(async move {
                match home.await {
                    Ok(Ok(home)) => {
                        EitherOf3::A(
                            view! {
                                <section>
                                    <ParagraphsByMultiline text=home.home_text />
                                </section>
                            },
                        )
                    }
                    Ok(Err(e)) => EitherOf3::B(view! { <AlertDanger text=e.to_string() /> }),
                    Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
                }
            })}
        </Suspense>
        <hr />
        <PostList />
    }
}

#[server(GetHome, "/api")]
pub async fn get_home() -> Result<Result<HomeData, AppError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;
    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({ home_text }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Settings find"))?;
    let Some(settings) = settings else {
        tracing::error!("settings record not found in database");
        crate::server::serverr_404();
        return Ok(Err(AppError::NotFound));
    };

    let home_data = HomeData {
        home_text: settings.home_text,
    };
    Ok(Ok(home_data))
}
