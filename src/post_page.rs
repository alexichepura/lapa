use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    img::{ImgData, Thumb},
    util::Loading,
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
    view! { cx,
        <Title text=post.title.clone()/>
        <Meta name="description" content=post.description/>
        <h1>{post.title}</h1>
        <hr/>
        <div class="post-images">
            <For
                each=move || post.images.clone()
                key=|image| image.id.clone()
                view=move |cx, image: ImgData| {
                    view! { cx, <Thumb image=image/> }
                }
            />
        </div>
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
