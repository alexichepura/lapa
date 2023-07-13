use leptos::{html::Dialog, *};
use leptos_meta::{Meta, Title};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    app::SettingsCx,
    img::{img_url_large, srcset_large},
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
}

#[component]
pub fn PostPage(cx: Scope) -> impl IntoView {
    let params = use_params::<PostParams>(cx);
    let slug = move || {
        params.with(|q| {
            q.as_ref()
                .map(|q| q.slug.clone())
                .map_err(|_| PostError::InvalidId)
        })
    };

    let post = create_blocking_resource(cx, slug, move |slug| async move {
        match slug {
            Err(e) => Err(e),
            Ok(slug) => get_post(cx, slug)
                .await
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });

    view! { cx,
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                post.read(cx)
                    .map(|post| match post {
                        Err(e) => view! { cx, <p>{e.to_string()}</p> }.into_view(cx),
                        Ok(post) => view! { cx, <PostView post=post/> }.into_view(cx),
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn PostView(cx: Scope, post: PostData) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = create_node_ref(cx);
    let (dialog_open, set_dialog_open) = create_signal::<DialogSignal>(cx, None);

    create_effect(cx, move |old| {
        let current = dialog_open();
        if let Some(_id) = current.clone() {
            let el = dialog_element().expect("<dialog> to exist");
            let _modal_result = el.show_modal();
        } else {
            if old.is_some() {
                // calling ref reruns effect, so need to check old value
                let el = dialog_element();
                if let Some(el) = el {
                    let _modal_result = el.close();
                }
            }
        }
        current
    });

    let dialog_view = move || match dialog_open() {
        Some(image) => view! { cx, <PostImageModal image set_dialog_open/> }.into_view(cx),
        None => ().into_view(cx),
    };
    view! { cx,
        <Title text=post.title.clone()/>
        <Meta name="description" content=post.description/>
        <h1>{post.title}</h1>
        <section><ParagraphsByMultiline text=post.text/></section>
        <hr/>
        <div class="post-images">
            <For
                each=move || post.images.clone()
                key=|image| image.id.clone()
                view=move |cx, image: ImgData| {
                    view! { cx, <Thumb image=image set_dialog_open/> }
                }
            />
        </div>
        <dialog node_ref=dialog_element>
            {dialog_view}
        </dialog>
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImgData {
    pub id: String,
    pub alt: String,
}

type DialogSignal = Option<ImgData>;
#[component]
pub fn Thumb(
    cx: Scope,
    image: ImgData,
    set_dialog_open: WriteSignal<DialogSignal>,
) -> impl IntoView {
    let settings = use_context::<SettingsCx>(cx).expect("to have found the settings provided");
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
    view! { cx,
        <figure>
            <img
                on:click=on_edit
                src=src
                srcset=srcset
                width=settings.thumb_width
                height=settings.thumb_height
            />
            <figcaption>{&image.alt}</figcaption>
        </figure>
    }
}

#[component]
pub fn PostImageModal(
    cx: Scope,
    image: ImgData,
    set_dialog_open: WriteSignal<DialogSignal>,
) -> impl IntoView {
    view! { cx,
        <figure>
            <img src=img_url_large(&image.id) srcset=srcset_large(&image.id) />
            <figcaption>{&image.alt}</figcaption>
            <button on:click=move |ev| {
                ev.prevent_default();
                set_dialog_open(None);
            }>"Close"</button>
        </figure>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(
    cx: Scope,
    slug: String,
) -> Result<Result<PostData, PostError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let post = prisma_client
        .post()
        .find_unique(db::post::slug::equals(slug))
        .include(db::post::include!({
            images: select {
                id
                alt
            }
        }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

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
                true => Some(PostData {
                    id: post.id,
                    slug: post.slug,
                    title: post.title,
                    description: post.description,
                    text: post.text,
                    images: post
                        .images
                        .iter()
                        .map(|img| ImgData {
                            id: img.id.clone(),
                            alt: img.alt.clone(),
                        })
                        .collect(),
                }),
                false => None,
            }
        }
        None => None,
    };

    match result {
        Some(post) => Ok(Ok(post)),
        None => {
            crate::err::serverr_404(cx);
            Ok(Err(PostError::NotFound))
        }
    }
}
