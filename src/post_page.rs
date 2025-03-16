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
    settings::{use_settings, use_site_url},
    util::{AlertDanger, Loading, ParagraphsByMultiline},
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
            Err(e) => Ok(Err(e)),
            Ok(slug) => get_post(slug).await,
        }
    });

    view! {
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match post.await {
                    Ok(Ok(post)) => EitherOf3::A(view! { <PostView post=post /> }),
                    Ok(Err(e)) => EitherOf3::B(view! { <AlertDanger text=e.to_string() /> }),
                    Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn PostView(post: PostData) -> impl IntoView {
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
    // use prisma_client::db;
    // let prisma_client = crate::server::use_prisma()?;

    // let post = prisma_client
    //     .post()
    //     .find_unique(db::post::slug::equals(slug))
    //     .include(db::post::include!({
    //         images(vec![]).order_by(db::image::order::order(db::SortOrder::Asc)): select {
    //             id
    //             alt
    //             is_hero
    //         }
    //     }))
    //     .exec()
    //     .await
    //     .map_err(|e| lib::emsg(e, "Post find"))?;
    let db = crate::server::use_db()?;
    let post = crate::server::Post::filter(crate::server::Post::FIELDS.slug.eq(slug))
        .first(&db)
        .await
        .map_err(|e| crate::server::anyemsg(e, "Post find"))?;

    let Some(post) = post else {
        crate::server::serverr_404();
        return Ok(Err(PostError::NotFound));
    };
    // let Some(published) = post.published_at else {
    //     crate::server::serverr_404();
    //     return Ok(Err(PostError::NotFound));
    // };
    // let now = chrono::Utc::now().fixed_offset();
    // if published > now {
    //     crate::server::serverr_404();
    //     return Ok(Err(PostError::NotFound));
    // }
    let images = post
        .images()
        .collect::<Vec<_>>(&db)
        .await
        .map_err(|e| crate::server::anyemsg(e, "Post images"))?;

    let mut images_iter = images.iter();
    let hero: Option<String> = images_iter
        .find(|img| img.is_hero != 0)
        .map(|img| img.id.to_string().to_owned());

    let post_data = PostData {
        id: post.id.to_string(),
        slug: post.slug,
        title: post.title,
        description: post.description,
        text: post.text,
        hero,
        images: images_iter
            .map(|img| ImgData {
                id: img.id.to_string(),
                alt: img.alt.clone(),
            })
            .collect(),
    };

    Ok(Ok(post_data))
}
