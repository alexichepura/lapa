use leptos::{either::EitherOf3, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    post::{PostError, PostForm, PostFormData},
    util::{AlertDanger, Loading},
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: String,
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();
    // let id = move || {
    //     params.with(|q| {
    //         log!("{:?}", q);
    //         // Err(MissingParam("id")) when navigating away from page
    //         q.as_ref()
    //             .map(|q| q.id.clone())
    //             .map_err(|_| PostError::InvalidId)
    //     })
    // };
    let id = Memo::new(move |prev: Option<&Result<String, PostError>>| {
        params.with(|q| {
            // Memo to fix Err(MissingParam("id")) when navigating away from page inside <Outlet />
            // log!("{:?}", q);
            match q {
                Ok(q) => Ok(q.id.clone()),
                Err(_) => {
                    if let Some(Ok(prev)) = prev {
                        Ok(prev.to_owned())
                    } else {
                        Err(PostError::InvalidId)
                    }
                }
            }
        })
    });

    let post = Resource::new_blocking(
        move || id(),
        move |id| async move {
            match id {
                Err(e) => Ok(Err(e)),
                Ok(id) => get_post(id).await,
            }
        },
    );

    view! {
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match post.await {
                    Ok(Ok(post)) => EitherOf3::A(view! { <PostForm post=post /> }),
                    Ok(Err(e)) => EitherOf3::B(view! { <AlertDanger text=e.to_string() /> }),
                    Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
                }
            })}
        </Suspense>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Result<PostFormData, PostError>, ServerFnError> {
    let prisma_client = crate::server::use_prisma()?;

    let post = prisma_client
        .post()
        .find_unique(prisma_client::db::post::id::equals(id))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;

    let Some(post) = post else {
        crate::server::serverr_404();
        return Ok(Err(PostError::NotFound));
    };
    let post_data = PostFormData {
        id: Some(post.id),
        created_at: post.created_at,
        published_at: post.published_at,
        slug: post.slug,
        title: post.title,
        description: post.description,
        text: post.text,
    };
    Ok(Ok(post_data))
}
