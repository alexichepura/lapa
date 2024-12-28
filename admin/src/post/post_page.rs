use leptos::{either::EitherOf3, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::{
    post::{ImageStore, ImagesStore, PostError, PostForm, PostFormData},
    util::{AlertDanger, Loading},
};

use super::PostImageData;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: String,
}
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostPageData {
    form: PostFormData,
    images: Vec<PostImageData>,
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

    // let post_id_clone = post_id.clone();
    // let controller = ImagesController::default();
    // let ctrl_hero = controller.hero.clone();
    // let ctrl_hero2 = controller.hero.clone();

    // let images_res = Resource::new(
    //     move || {
    //         (
    //             id(),
    //             // controller.delete.version().get(),
    //             // controller.upload.version().get(),
    //             // controller.update.version().get(),
    //             // controller.order.version().get(),
    //             // controller.hero.version().get(),
    //             ctrl_hero2.version().get(),
    //         )
    //     },
    //     // move |(id, _, _, _, _, _)| async move {
    //     move |(id, _)| async move {
    //         tracing::info!("images res effect");
    //         match id {
    //             Err(e) => Ok(Err(e)),
    //             Ok(id) => get_images(id).await,
    //         }
    //     },
    // );
    // Effect::new(move |_| {
    //     let v = images_res.get();
    //     if let Some(v) = v {
    //         match v {
    //             Ok(Ok(images)) => controller.images.set(images),
    //             Ok(Err(_e)) => todo!(),
    //             Err(_) => todo!(),
    //         }
    //     }
    // });

    // Effect::new(move |prev| {
    //     tracing::info!("images effect");
    //     if let Some(_prev) = prev {
    //         images_res.refetch();
    //     }
    //     (
    //         // controller.delete.version().get(),
    //         // controller.upload.version().get(),
    //         // controller.update.version().get(),
    //         // controller.order.version().get(),
    //         ctrl_hero.version().get(),
    //     )
    // });

    view! {
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match post.await {
                    Ok(Ok(post)) => {
                        EitherOf3::A({
                            let images_store = Store::new(ImagesStore {
                                images: post
                                    .images
                                    .into_iter()
                                    .map(|image| ImageStore {
                                        id: image.id,
                                        alt: image.alt,
                                        order: image.order,
                                        is_hero: image.is_hero,
                                    })
                                    .collect(),
                            });
                            // let images = RwSignal::new(post.images);
                            // let post_images_rw = ArcRwSignal::new(post.images);
                            // let images_controller = ImagesController::new(post_images_rw);
                            // tracing::info!("images ctrl {:?}", images_controller.hero);
                            view! { <PostForm post=post.form images_store /> }
                        })
                    }
                    Ok(Err(e)) => {
                        EitherOf3::B(
                            // view! { <PostForm post=post.form images_controller=controller /> },
                            view! { <AlertDanger text=e.to_string() /> },
                        )
                    }
                    Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
                }
            })}
        </Suspense>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Result<PostPageData, PostError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let post = prisma_client
        .post()
        .find_unique(db::post::id::equals(id))
        .select(db::post::select!({
            id
            created_at
            published_at
            slug
            title
            description
            text
            images: select {
                id
                alt
                order
                is_hero
            }
        }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;

    let Some(post) = post else {
        crate::server::serverr_404();
        return Ok(Err(PostError::NotFound));
    };

    let form = PostFormData {
        id: Some(post.id),
        created_at: post.created_at,
        published_at: post.published_at,
        slug: post.slug,
        title: post.title,
        description: post.description,
        text: post.text,
    };
    let post_page_data = PostPageData {
        form,
        images: post
            .images
            .into_iter()
            .map(|img| PostImageData {
                id: img.id,
                alt: img.alt,
                order: img.order,
                is_hero: img.is_hero,
            })
            .collect(),
    };
    Ok(Ok(post_page_data))
}
