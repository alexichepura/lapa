use leptos::{either::EitherOf3, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

use crate::{
    post::{PostDeleteForm, PostError, PostForm, PostFormData}, util::{AlertDanger, Loading}
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: String,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostPageData {
    form: PostFormData,
    content_id: String,
    content_json: String,
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

    let suspended = move || Suspend::new(async move {
        match post.await {
            Ok(Ok(page)) => EitherOf3::A(view! { <PostPageView page=page /> }),
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
pub fn PostPageView(page: PostPageData) -> impl IntoView {
    let (hydrated, set_hydrated) = signal(false);
    Effect::new(move |_| {
        set_hydrated(true);
    });
    let edit_view = {
        move || match hydrated() {
            true => leptos::either::Either::Left(
                #[cfg(feature = "hydrate")]
                view! {
                    <crate::content::ContentHtml
                        content_id=page.content_id.clone()
                        content_json=page.content_json.clone()
                    />
                },
                #[cfg(not(feature = "hydrate"))]
                view! {  },
            ),
            false => leptos::either::Either::Right(()),
        }
    };
    let id = page.form.id.clone();
    let slug = page.form.slug.clone();
    view! {
        <PostForm post=page.form />
        {edit_view}
        <div class="Grid-fluid-2">
            <PostDeleteForm id=id.clone() slug />
        </div>
    }
}


#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Result<PostPageData, PostError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let page = clorinde::queries::admin_post::page()
        .bind(&db, &id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;
    let Some(page) = page else {
        crate::server::serverr_404();
        return Ok(Err(PostError::NotFound));
    };
    let form = PostFormData {
        id: page.id,
        created_at: page.created_at.and_utc().fixed_offset(),
        publish_at: page.publish_at.map(|dt| dt.and_utc().fixed_offset()),
        slug: page.slug,
        title: page.meta_title,
        description: page.meta_description,
    };
    let page = PostPageData {
        form,
        content_id: page.content_id,
        content_json: page.content_json,
    };
    Ok(Ok(page))
}
