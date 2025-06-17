use leptos::{
    either::EitherOf3,
    prelude::*,
};
use leptos_meta::{Meta, Title};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    settings::{use_site_url},
    util::{AlertDanger, Loading},
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    category_slug: String,
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
    pub content: String,
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();
    let params_fn = move || {
        params.with(|q| {
            q.as_ref()
                .map(|q| (q.category_slug.clone(), q.slug.clone()))
                .map_err(|_| PostError::InvalidId)
        })
    };

    let post = Resource::new_blocking(params_fn, move |params_result| async move {
        match params_result {
            Err(e) => Ok(Err(e)),
            Ok((category, slug)) => get_post(category, slug).await,
        }
    });

    let suspended = move || Suspend::new(async move {
        match post.await {
            Ok(Ok(post)) => EitherOf3::A(view! { <PostView post=post /> }),
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
pub fn PostView(post: PostData) -> impl IntoView {
    view! {
        <Title text=post.title.clone() />
        <Meta name="description" content=post.description.clone() />
        <Meta property="og:title" content=post.title.clone() />
        <Meta property="og:description" content=post.description.clone() />
        <h1>{post.title}</h1>
        <article inner_html=post.content></article>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(
    category_slug: String,
    slug: String
) -> Result<Result<PostData, PostError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let post = clorinde::queries::post::page()
        .bind(&db, &slug).opt()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;
    let Some(post) = post else {
        crate::server::serverr_404();
        return Ok(Err(PostError::NotFound));
    };
    // let (html, headings) = content::content_json_to_html_with_headings(&post.content.json);
    let html = content::content_json_to_html(&post.content_json);

    let post_data = PostData {
        id: post.id,
        slug: post.slug,
        title: post.meta_title,
        description: post.meta_description,
        content: html,
    };

    Ok(Ok(post_data))
}
