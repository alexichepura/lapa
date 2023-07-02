use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{post::ImageUpload, util::Loading};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostImageData {
    pub id: String,
    pub alt: String,
}

#[component]
pub fn PostImages(cx: Scope, post_id: String) -> impl IntoView {
    let post_id_clone = post_id.clone();
    let images = create_blocking_resource(
        cx,
        move || (post_id_clone.clone()),
        move |post_id| get_images(cx, post_id),
    );
    // let images = create_blocking_resource(
    //     cx,
    //     move || (post_id.clone()),
    //     move |post_id| get_images(cx, post_id),
    // );

    view! { cx,
        <ImageUpload post_id=post_id/>
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                images
                    .read(cx)
                    .map(|images| match images {
                        Err(e) => view! { cx, <p>"error" {e.to_string()}</p> }.into_view(cx),
                        Ok(images) => {
                            if images.is_empty() {
                                view! { cx, <p>"No images were found."</p> }.into_view(cx)
                            } else {
                                view! { cx, <PostImagesView images=images/> }.into_view(cx)
                            }
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn PostImagesView(cx: Scope, images: Vec<PostImageData>) -> impl IntoView {
    view! { cx,
        <h2>"Images"</h2>
        <div class="Grid-fluid-2">
            <For
                each=move || images.clone()
                key=|image| image.id.clone()
                view=move |cx, image: PostImageData| {
                    view! { cx, <PostImage image=image/> }
                }
            />
        </div>
    }
}

#[component]
pub fn PostImage(cx: Scope, image: PostImageData) -> impl IntoView {
    let src = format!("/img/{}-s.webp", image.id);
    let srcset = format!("/img/{}-s2.webp 2x", image.id);
    view! { cx,
        <div class="Card">
            <img src=src srcset=srcset width=250/>
            <div>"Alt: " {image.alt}</div>
        </div>
    }
}

#[server(GetImages, "/api")]
pub async fn get_images(cx: Scope, post_id: String) -> Result<Vec<PostImageData>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;
    let images = prisma_client
        .image()
        .find_many(vec![db::image::WhereParam::PostId(
            db::read_filters::StringFilter::Equals(post_id),
        )])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let images: Vec<PostImageData> = images
        .iter()
        .map(|data| PostImageData {
            id: data.id.clone(),
            alt: data.alt.clone(),
        })
        .collect();
    Ok(images)
}
