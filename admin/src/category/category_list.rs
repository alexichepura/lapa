use chrono::{DateTime, FixedOffset};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::util::{datetime_to_strings, AlertDanger, Loading};

#[component]
pub fn CategoryList() -> impl IntoView {
    let categories = Resource::new_blocking(|| (), move |_| get_categories());

    view! {
        <Title text="Categories" />
        <h1>
            <span>"Categories"</span>
            <span>" "</span>
            <A href="/post-category/new">"+1"</A>
        </h1>

        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match categories.await {
                    Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
                    Ok(categories) => Either::Right(view! { <CategoryListItems categories /> }),
                }
            })}
        </Suspense>
    }
    .into_any()
}
#[component]
pub fn CategoryListItems(categories: Vec<CategoryListItem>) -> impl IntoView {
    if categories.is_empty() {
        Either::Left(view! { <p>"0 categories"</p> })
    } else {
        Either::Right(view! {
            <ul class="Card Listing">
                {categories
                    .into_iter()
                    .map(|category| {
                        view! { <CategoryListItem category /> }
                    })
                    .collect_view()}

            </ul>
        })
    }
}

#[component]
pub fn CategoryListItem(category: CategoryListItem) -> impl IntoView {
    let created = datetime_to_strings(category.created_at);
    view! {
        <li>
            <div>{created.local}</div>
            <A href=format!("/post-category/{}", category.id)>{category.slug}</A>
            <div>{category.name}</div>
        </li>
    }
}

#[server(GetCategoriesList, "/api")]
async fn get_categories() -> Result<Vec<CategoryListItem>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let categories = clorinde::queries::admin_post_category::list()
        .bind(&db)
        .map(|data| CategoryListItem {
            id: data.id.into(),
            created_at: data.created_at.and_utc().fixed_offset(),
            slug: data.slug.into(),
            name: data.name.into(),
        })
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Post category list"))?;
    Ok(categories)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryListItem {
    pub id: String,
    pub created_at: DateTime<FixedOffset>,
    pub slug: String,
    pub name: String,
}
