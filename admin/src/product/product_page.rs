use leptos::{either::EitherOf3, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    product::{ProductError, ProductForm, ProductFormData},
    util::{AlertDanger, Loading},
};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct ProductParams {
    id: String,
}

#[component]
pub fn ProductPage() -> impl IntoView {
    let params = use_params::<ProductParams>();
    // let id = move || {
    //     params.with(|q| {
    //         log!("{:?}", q);
    //         // Err(MissingParam("id")) when navigating away from page
    //         q.as_ref()
    //             .map(|q| q.id.clone())
    //             .map_err(|_| PostError::InvalidId)
    //     })
    // };
    let id = Memo::new(move |prev: Option<&Result<String, ProductError>>| {
        params.with(|q| {
            // Memo to fix Err(MissingParam("id")) when navigating away from page inside <Outlet />
            // log!("{:?}", q);
            match q {
                Ok(q) => Ok(q.id.clone()),
                Err(_) => {
                    if let Some(Ok(prev)) = prev {
                        Ok(prev.to_owned())
                    } else {
                        Err(ProductError::InvalidId)
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
                    Ok(Ok(post)) => EitherOf3::A(view! { <ProductForm product=post /> }),
                    Ok(Err(e)) => EitherOf3::B(view! { <AlertDanger text=e.to_string() /> }),
                    Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
                }
            })}
        </Suspense>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Result<ProductFormData, ProductError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let post = clorinde::queries::admin_product::page()
        .bind(&db, &id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;
    let Some(post) = post else {
        crate::server::serverr_404();
        return Ok(Err(ProductError::NotFound));
    };
    let post_data = ProductFormData {
        id: post.id,
        created_at: post.created_at.and_utc().fixed_offset(),
        publish_at: post.publish_at.map(|dt| dt.and_utc().fixed_offset()),
        slug: post.slug,
        title: post.meta_title,
        description: post.meta_description,
    };
    Ok(Ok(post_data))
}
