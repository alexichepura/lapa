use leptos::{either::EitherOf3, prelude::*};
use leptos_meta::{Meta, Title};
use serde::{Deserialize, Serialize};

use crate::{
    err::AppError,
    product_list::ProductList,
    util::{AlertDanger, ParagraphsByMultiline},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomeData {
    pub home_text: String,
}

#[component]
pub fn HomePage() -> impl IntoView {
    let home = Resource::new_blocking(|| (), move |_| get_home());
    let suspended = move || Suspend::new(async move {
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
    });
    view! {
        <Title text="Home" />
        <Meta
            name="description"
            content="Leptos Axum starter with Admin dashboard and SSR/SPA website"
        />
        <h1>Welcome to Lapa</h1>
        <Suspense>{suspended}</Suspense>
        <hr />
        <ProductList />
    }
}

#[server(GetHome, "/api")]
pub async fn get_home() -> Result<Result<HomeData, AppError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let home = clorinde::queries::settings::settings_home().bind(&db).opt().await.unwrap();
    let home = home.unwrap();

    let home_data = HomeData {
        home_text: home,
    };
    Ok(Ok(home_data))
}
