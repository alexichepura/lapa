use chrono::{DateTime, FixedOffset};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    image::img_url_small,
    util::{datetime_to_strings, AlertDanger, DateTimeStrings, Loading},
};

#[component]
pub fn ProductList() -> impl IntoView {
    let products = Resource::new_blocking(|| (), move |_| get_products());

    view! {
        <Title text="Products" />
        <h1>
            <span>Products</span>
            <small>
                <A href="/product/new">Create</A>
            </small>
        </h1>
        <ul class="Card Listing">
            <Suspense fallback=move || {
                view! { <Loading /> }
            }>
                {move || Suspend::new(async move {
                    match products.await {
                        Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
                        Ok(products) => Either::Right(view! { <PostListItems products /> }),
                    }
                })}
            </Suspense>
        </ul>
    }
}
#[component]
pub fn PostListItems(products: Vec<ProductListItem>) -> impl IntoView {
    if products.is_empty() {
        Either::Left(view! { <p>No products were found.</p> })
    } else {
        Either::Right(
            products
                .into_iter()
                .map(|product| {
                    view! { <ProductListItem product /> }
                })
                .collect_view(),
        )
    }
}

#[component]
pub fn ProductListItem(product: ProductListItem) -> impl IntoView {
    let created = datetime_to_strings(product.created_at);
    let publish_at_view = match product.publish_at {
        Some(publish_at) => datetime_to_strings(publish_at),
        None => DateTimeStrings::draft(),
    };
    let class = match product.is_published {
        true => "published",
        false => "not-published",
    };
    let hero_view = match product.hero {
        Some(id) => {
            Either::Left(view! { <img title="Post hero" src=img_url_small(&id) width="36" /> })
        }
        None => Either::Right(view! { <div title="No post hero">?</div> }),
    };
    view! {
        <li class="PostListItem">
            <A href=format!("/product/{}", product.id)>
                <div title="Publish at" class=format!("PostListItem-status {}", class)>
                    {publish_at_view.utc}
                </div>
                {hero_view}
                <span title="Post title">{product.h1}</span>
                <div title="Created at" class="PostListItem-created">
                    {created.utc}
                </div>
            </A>
        </li>
    }
}

#[server(GetProducts, "/api")]
pub async fn get_products() -> Result<Vec<ProductListItem>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let products = clorinde::queries::admin_product::list()
        .bind(&db)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Post list find"))?;
    let products: Vec<ProductListItem> = products
        .into_iter()
        .map(|data| {
            let is_published: bool = match data.publish_at {
                Some(published_at) => chrono::Utc::now().fixed_offset() > published_at.and_utc().fixed_offset(),
                None => false,
            };
            ProductListItem {
                id: data.id,
                h1: data.h1,
                created_at: data.created_at.and_utc().fixed_offset(),
                publish_at: data.publish_at.map(|dt| dt.and_utc().fixed_offset()),
                is_published,
                hero: data.image_id,
            }
        })
        .collect();
    Ok(products)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductListItem {
    pub id: String,
    pub h1: String,
    pub created_at: DateTime<FixedOffset>,
    pub publish_at: Option<DateTime<FixedOffset>>,
    pub is_published: bool,
    pub hero: Option<String>,
}
