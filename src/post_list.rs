use leptos::*;
use leptos_router::A;
use serde::{Deserialize, Serialize};

use crate::{
    img::{ImgData, Thumb},
    util::Loading,
};

#[component]
pub fn PostList() -> impl IntoView {
    let posts = create_blocking_resource(|| (), move |_| get_posts());

    view! {
        <h2>Posts</h2>
        <Suspense fallback=move || {
            view! { <Loading/> }
        }>
            {move || {
                posts
                    .read()
                    .map(|posts| match posts {
                        Err(e) => {
                            view! { <p>error {e.to_string()}</p> }
                                .into_view()
                        }
                        Ok(posts) => {
                            view! { <PostListView posts/> }
                                .into_view()
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn PostListView(posts: Vec<PostListItem>) -> impl IntoView {
    let list = if posts.is_empty() {
        view! { <p>No posts were found.</p> }.into_view()
    } else {
        posts
            .into_iter()
            .map(|post| {
                view! { <PostListItem post/> }
            })
            .collect_view()
    };
    view! { <div class="PostList">{list}</div> }
}

#[component]
pub fn PostListItem(post: PostListItem) -> impl IntoView {
    let image_view = match post.hero {
        Some(image) => view! { <Thumb image/> }.into_view(),
        None => ().into_view(),
    };
    let href = format!("/post/{}", post.slug);
    view! { <A href>{image_view} <p>{&post.title}</p></A> }
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<PostListItem>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;
    let now = prisma_client_rust::chrono::Utc::now().fixed_offset();
    let posts = prisma_client
        .post()
        .find_many(vec![db::post::published_at::lt(now)])
        .include(db::post::include!({
            images(vec![db::image::is_hero::equals(true)]).take(1): select {
                id alt
            }
        }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let posts: Vec<PostListItem> = posts
        .iter()
        .map(|data| {
            let hero = data.images.first().map(|image| ImgData {
                id: image.id.clone(),
                alt: image.alt.clone(),
            });
            PostListItem {
                id: data.id.clone(),
                title: data.title.clone(),
                slug: data.slug.clone(),
                hero,
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
    pub hero: Option<ImgData>,
}
