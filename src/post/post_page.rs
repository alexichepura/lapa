use leptos::{
    either::EitherOf3,
    prelude::*,
};
use leptos_meta::{Meta, Title};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
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
    pub meta_title: String,
    pub meta_description: String,
    pub h1: String,
    pub content_html: String,
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
        <Title text=post.meta_title.clone() />
        <Meta name="description" content=post.meta_description.clone() />
        <Meta property="og:title" content=post.meta_title.clone() />
        <Meta property="og:description" content=post.meta_description.clone() />
        <h1>{post.h1}</h1>
        <article inner_html=post.content_html></article>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(
    category_slug: String,
    slug: String
) -> Result<Result<PostData, PostError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let page = clorinde::queries::post::page()
        .bind(&db, &category_slug, &slug).opt()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;
    let Some(page) = page else {
        crate::server::serverr_404();
        return Ok(Err(PostError::NotFound));
    };
    // let (html, headings) = content::content_json_to_html_with_headings(&post.content.json);
    let content_html = content::content_json_to_html(&page.content_json);

    let post_data = PostData {
        id: page.id,
        slug: page.slug,
        meta_title: page.meta_title,
        meta_description: page.meta_description,
        h1: page.h1,
        content_html,
    };

    Ok(Ok(post_data))
}
