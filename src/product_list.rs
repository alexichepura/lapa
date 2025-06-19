use leptos::{either::Either, prelude::*};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    util::{AlertDanger, Loading},
};

#[component]
pub fn ProductList() -> impl IntoView {
    let products = Resource::new_blocking(|| (), move |_| get_products());
    let suspended = move || Suspend::new(async move {
        match products.await {
            Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
            Ok(products) => Either::Right(view! { <ProductListView products /> }),
        }
    });
    view! {
        <h2>Products</h2>
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>{suspended}</Suspense>
    }
}

#[component]
pub fn ProductListView(products: Vec<ProductListItem>) -> impl IntoView {
    let list = if products.is_empty() {
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
    };
    view! { <div class="ProductList">{list}</div> }
}

#[component]
pub fn ProductListItem(product: ProductListItem) -> impl IntoView {
    let image_view = match product.hero {
        Some(image) => {
            let src = format!("/product-image/{}_s", image.id);
            Either::Left(view! { <img src=src alt=image.alt /> })
        },
        None => Either::Right(()),
    };
    let href = format!("/product/{}", product.slug);
    view! { <A href>{image_view} <p>{product.h1}</p></A> }
}

#[server(GetProducts, "/api")]
pub async fn get_products() -> Result<Vec<ProductListItem>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let products = clorinde::queries::product::list()
        .bind(&db).all()
        .await
        .map_err(|e| lib::emsg(e, "Products list find"))?;
    let products: Vec<ProductListItem> = products
        .into_iter()
        .map(|data| {
            let hero = match (data.image_id, data.alt) {
                (Some(id), Some(alt)) => Some(ImgData { id, alt }),
                _ => None,
            };
            ProductListItem {
                id: data.id,
                h1: data.h1,
                slug: data.slug,
                hero,
            }
        })
        .collect();
    Ok(products)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductListItem {
    pub id: String,
    pub h1: String,
    pub slug: String,
    pub hero: Option<ImgData>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImgData {
    pub id: String,
    pub alt: String,
}
