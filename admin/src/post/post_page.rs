use leptos::*;
use leptos_router::*;

use crate::{
    post::{PostError, PostForm, PostFormData},
    util::Loading,
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: String,
}

#[component]
pub fn PostPage(cx: Scope) -> impl IntoView {
    let params = use_params::<PostParams>(cx);
    let id = move || {
        params.with(|q| {
            q.as_ref()
                .map(|q| q.id.clone())
                .map_err(|_| PostError::InvalidId)
        })
    };

    let post = create_blocking_resource(cx, id, move |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(cx, id)
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
                        Err(e) => {
                            view! { cx, <p>{e.to_string()}</p> }
                                .into_view(cx)
                        }
                        Ok(post) => {
                            view! { cx, <PostForm post=post/> }
                                .into_view(cx)
                        }
                    })
            }}
        </Suspense>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(
    cx: Scope,
    id: String,
) -> Result<Result<PostFormData, PostError>, ServerFnError> {
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let post = prisma_client
        .post()
        .find_unique(prisma_client::db::post::id::equals(id))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(match post {
        Some(post) => Ok(PostFormData {
            id: Some(post.id),
            created_at: post.created_at,
            published_at: post.published_at,
            slug: post.slug,
            title: post.title,
            description: post.description,
            text: post.text,
        }),
        None => {
            crate::err::serverr_404(cx);
            Err(PostError::NotFound)
        }
    })
}
