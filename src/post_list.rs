use leptos::*;
use leptos_router::A;
use serde::{Deserialize, Serialize};

use crate::{
    img::{ImgData, Thumb},
    util::Loading,
};

#[component]
pub fn PostList(cx: Scope) -> impl IntoView {
    let posts = create_blocking_resource(cx, || (), move |_| get_posts(cx));

    view! { cx,
        <h2>"Posts"</h2>
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                posts
                    .read(cx)
                    .map(|posts| match posts {
                        Err(e) => {
                            view! { cx, <p>"error" {e.to_string()}</p> }
                                .into_view(cx)
                        }
                        Ok(posts) => {
                            view! { cx, <PostListView posts/> }
                                .into_view(cx)
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn PostListView(cx: Scope, posts: Vec<PostListItem>) -> impl IntoView {
    let list = if posts.is_empty() {
        view! { cx, <p>"No posts were found."</p> }.into_view(cx)
    } else {
        posts
            .into_iter()
            .map(|post| {
                view! { cx, <PostListItem post/> }
            })
            .collect_view(cx)
    };
    view! { cx, <div class="PostList">{list}</div> }
}

#[component]
pub fn PostListItem(cx: Scope, post: PostListItem) -> impl IntoView {
    let image_view = match post.image {
        Some(image) => view! { cx, <Thumb image/> }.into_view(cx),
        None => ().into_view(cx),
    };
    let href = format!("/post/{}", post.slug);
    view! { cx, <A href>{image_view} <p>{&post.title}</p></A> }
}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope) -> Result<Vec<PostListItem>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;
    let posts = prisma_client
        .post()
        .find_many(vec![])
        .include(db::post::include!({ images(vec![]).take(1): select {
            id
            alt
        } }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let posts: Vec<PostListItem> = posts
        .iter()
        .map(|data| {
            let image = data.images.first();
            let image = match image {
                Some(image) => Some(ImgData {
                    id: image.id.clone(),
                    alt: image.alt.clone(),
                }),
                None => None,
            };
            // let (image_id, image_alt): (String, String) = match image
            PostListItem {
                id: data.id.clone(),
                title: data.title.clone(),
                slug: data.slug.clone(),
                image,
            }
        })
        .collect();
    Ok(posts)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostListItem {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub image: Option<ImgData>,
}
