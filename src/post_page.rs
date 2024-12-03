use leptos::{either::Either, html::Dialog, prelude::*};
use leptos_meta::{Meta, Title};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    img::{img_url_large, img_url_large_retina, srcset_large},
    settings::{use_settings, use_site_url},
    util::{Loading, ParagraphsByMultiline},
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    slug: String,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidId,
    #[error("Post not found.")]
    NotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostData {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub text: String,
    pub images: Vec<ImgData>,
    pub hero: Option<String>,
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();
    let slug = move || {
        params.with(|q| {
            q.as_ref()
                .map(|q| q.slug.clone())
                .map_err(|_| PostError::InvalidId)
        })
    };

    let post = Resource::new_blocking(slug, move |slug| async move {
        match slug {
            Err(e) => Err(e),
            Ok(slug) => get_post(slug)
                .await
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });

    view! {
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || {
                post.get()
                    .map(|post| match post {
                        Err(e) => Either::Left(view! { <p>{e.to_string()}</p> }),
                        Ok(post) => Either::Right(view! { <PostView post=post /> }),
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn PostView(post: PostData) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let (dialog_open, set_dialog_open) = signal::<DialogSignal>(None);

    Effect::new(move |old| {
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
        Some(image) => Either::Left(view! { <PostImageModal image set_dialog_open /> }),
        None => Either::Right(()),
    };

    let site_url = use_site_url();

    let hero_og = match post.hero {
        Some(hero) => {
            let og = format!("{site_url}{}", img_url_large_retina(&hero)); // TODO domain from DB
            Either::Left(view! { <Meta property="og:image" content=og /> })
        }
        None => Either::Right(()),
    };

    view! {
        <Title text=post.title.clone() />
        <Meta name="description" content=post.description.clone() />
        <Meta property="og:title" content=post.title.clone() />
        <Meta property="og:description" content=post.description.clone() />
        {hero_og}
        <h1>{post.title}</h1>
        <section>
            <ParagraphsByMultiline text=post.text />
        </section>
        <hr />
        <div class="post-images">
            <For
                each=move || post.images.clone()
                key=|image| image.id.clone()
                children=move |image: ImgData| {
                    view! { <Thumb image=image set_dialog_open /> }
                }
            />

        </div>
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
    let settings = use_settings();
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
            <img
                on:click=on_edit
                src=src
                srcset=srcset
                width=settings.thumb_width
                height=settings.thumb_height
            />
            <figcaption>{image.alt}</figcaption>
        </figure>
    }
}

#[component]
pub fn PostImageModal(image: ImgData, set_dialog_open: WriteSignal<DialogSignal>) -> impl IntoView {
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

#[server(GetPost, "/api")]
pub async fn get_post(slug: String) -> Result<Result<PostData, PostError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let post = prisma_client
        .post()
        .find_unique(db::post::slug::equals(slug))
        .include(db::post::include!({
            images(vec![]).order_by(db::image::order::order(db::SortOrder::Asc)): select {
                id
                alt
                is_hero
            }
        }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;

    let result: Option<PostData> = match post {
        Some(post) => {
            let published = match post.published_at {
                Some(published_at) => {
                    let now = chrono::Utc::now().fixed_offset();
                    published_at < now
                }
                None => false,
            };
            match published {
                true => {
                    let hero = post
                        .images
                        .clone()
                        .into_iter()
                        .find(|img| img.is_hero)
                        .map(|img| img.id);
                    Some(PostData {
                        id: post.id,
                        slug: post.slug,
                        title: post.title,
                        description: post.description,
                        text: post.text,
                        hero,
                        images: post
                            .images
                            .iter()
                            .map(|img| ImgData {
                                id: img.id.clone(),
                                alt: img.alt.clone(),
                            })
                            .collect(),
                    })
                }
                false => None,
            }
        }
        None => None,
    };

    match result {
        Some(post) => Ok(Ok(post)),
        None => {
            crate::server::serverr_404();
            Ok(Err(PostError::NotFound))
        }
    }
}
