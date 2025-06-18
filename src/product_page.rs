use leptos::{
    either::{Either, EitherOf3},
    html::Dialog,
    prelude::*,
};
use leptos_meta::{Meta, Title};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    img::{img_url_large, img_url_large_retina, srcset_large},
    settings::{use_site_url},
    util::{AlertDanger, Loading, ParagraphsByMultiline},
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct ProductParams {
    slug: String,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductError {
    #[error("Invalid product ID.")]
    InvalidId,
    #[error("Product not found.")]
    NotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductPageData {
    pub id: String,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub h1: String,
    pub content_html: String,
    pub images: Vec<ImgData>,
    pub hero: Option<String>,
}

#[component]
pub fn ProductPage() -> impl IntoView {
    let params = use_params::<ProductParams>();
    let slug = move || {
        params.with(|q| {
            q.as_ref()
                .map(|q| q.slug.clone())
                .map_err(|_| ProductError::InvalidId)
        })
    };
    let product = Resource::new_blocking(slug, move |slug| async move {
        match slug {
            Err(e) => Ok(Err(e)),
            Ok(slug) => get_product(slug).await,
        }
    });
    let suspended = move || Suspend::new(async move {
        match product.await {
            Ok(Ok(product)) => EitherOf3::A(view! { <ProductPageView product=product /> }),
            Ok(Err(e)) => EitherOf3::B(view! { <AlertDanger text=e.to_string() /> }),
            Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
        }
    });
    view! {
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>{suspended}</Suspense>
    }
}

#[component]
pub fn ProductPageView(product: ProductPageData) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let (dialog_open, set_dialog_open) = signal::<DialogSignal>(None);

    Effect::new(move |old: Option<Option<ImgData>>| {
        let current = dialog_open();
        if let Some(_id) = current.clone() {
            let el = dialog_element.get().expect("<dialog> to exist");
            let _modal_result = el.show_modal();
        } else {
            if old.is_some() {
                // calling ref reruns effect, so need to check old value
                let el = dialog_element.get();
                if let Some(el) = el {
                    let _modal_result = el.close();
                }
            }
        }
        current
    });

    let dialog_view = move || match dialog_open() {
        Some(image) => Either::Left(view! { <ProductImageModal image set_dialog_open /> }),
        None => Either::Right(()),
    };

    let site_url = use_site_url();

    let hero_og = match product.hero {
        Some(hero) => {
            let og = format!("{site_url}{}", img_url_large_retina(&hero)); // TODO domain from DB
            Either::Left(view! { <Meta property="og:image" content=og /> })
        }
        None => Either::Right(()),
    };

    view! {
        <Title text=product.meta_title.clone() />
        <Meta name="description" content=product.meta_description.clone() />
        <Meta property="og:title" content=product.meta_title.clone() />
        <Meta property="og:description" content=product.meta_description.clone() />
        {hero_og}
        <h1>{product.meta_title}</h1>
        <div class="product-images">
            <For
                each=move || product.images.clone()
                key=|image| image.id.clone()
                children=move |image: ImgData| {
                    view! { <Thumb image=image set_dialog_open /> }
                }
            />

        </div>
        <article inner_html=product.content_html></article>
        <dialog node_ref=dialog_element>{dialog_view}</dialog>
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImgData {
    pub id: String,
    pub alt: String,
}

type DialogSignal = Option<ImgData>;
#[component]
pub fn Thumb(image: ImgData, set_dialog_open: WriteSignal<DialogSignal>) -> impl IntoView {
    let id = image.id.clone();
    let alt = image.alt.clone();

    let on_edit = move |_| {
        set_dialog_open(Some(ImgData {
            id: id.clone(),
            alt: alt.clone(),
        }));
    };

    let src = format!("/img/{}-s.webp", image.id);
    let srcset = format!("/img/{}-s2.webp 2x", image.id);
    view! {
        <figure>
            <img on:click=on_edit src=src srcset=srcset />
            <figcaption>{image.alt}</figcaption>
        </figure>
    }
}

#[component]
pub fn ProductImageModal(image: ImgData, set_dialog_open: WriteSignal<DialogSignal>) -> impl IntoView {
    view! {
        <figure>
            <img src=img_url_large(&image.id) srcset=srcset_large(&image.id) />
            <figcaption>{image.alt}</figcaption>
            <button on:click=move |ev| {
                ev.prevent_default();
                set_dialog_open(None);
            }>Close</button>
        </figure>
    }
}

#[server(GetProduct, "/api")]
pub async fn get_product(slug: String) -> Result<Result<ProductPageData, ProductError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let page = clorinde::queries::product::page()
        .bind(&db, &slug).opt()
        .await
        .map_err(|e| lib::emsg(e, "Product find"))?;
    let Some(page) = page else {
        crate::server::serverr_404();
        return Ok(Err(ProductError::NotFound));
    };
    let images = clorinde::queries::product::images()
        .bind(&db, &page.id).all()
        .await
        .map_err(|e| lib::emsg(e, "Product images find"))?;
    let hero = images
        .iter()
        .find(|img| img.is_hero)
        .map(|img| img.id.clone());
    // let (html, headings) = content::content_json_to_html_with_headings(&page.content.json);
    let content_html = content::content_json_to_html(&page.content_json);

    let page = ProductPageData {
        id: page.id,
        slug: page.slug,
        meta_title: page.meta_title,
        meta_description: page.meta_description,
        h1: page.h1,
        content_html,
        hero,
        images: images
            .into_iter()
            .map(|img| ImgData {
                id: img.id,
                alt: img.alt,
            })
            .collect(),
    };
    Ok(Ok(page))
}
