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
        <h2>Posts</h2>
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                posts
                    .read(cx)
                    .map(|posts| match posts {
                        Err(e) => {
                            view! { cx, <p>error {e.to_string()}</p> }
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
        view! { cx, <p>No posts were found.</p> }.into_view(cx)
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
    let image_view = match post.hero {
        Some(image) => view! { cx, <Thumb image/> }.into_view(cx),
        None => ().into_view(cx),
    };
    let href = format!("/post/{}", post.slug);
    view! { cx, <A href>{image_view} <p>{&post.title}</p></A> }
}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope) -> Result<Vec<PostListItem>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma(cx)?;
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
