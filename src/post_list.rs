use leptos::{either::Either, prelude::*};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    img::{ImgData, Thumb},
    util::{AlertDanger, Loading},
};

#[component]
pub fn PostList() -> impl IntoView {
    let posts = Resource::new_blocking(|| (), move |_| get_posts());

    view! {
        <h2>Posts</h2>
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match posts.await {
                    Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
                    Ok(posts) => Either::Right(view! { <PostListView posts /> }),
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn PostListView(posts: Vec<PostListItem>) -> impl IntoView {
    let list = if posts.is_empty() {
        Either::Left(view! { <p>No posts were found.</p> })
    } else {
        Either::Right(
            posts
                .into_iter()
                .map(|post| {
                    view! { <PostListItem post /> }
                })
                .collect_view(),
        )
    };
    view! { <div class="PostList">{list}</div> }
}

#[component]
pub fn PostListItem(post: PostListItem) -> impl IntoView {
    let image_view = match post.hero {
        Some(image) => Either::Left(view! { <Thumb image /> }),
        None => Either::Right(()),
    };
    let href = format!("/post/{}", post.slug);
    view! { <A href>{image_view} <p>{post.title}</p></A> }
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<PostListItem>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let posts = clorinde::queries::post::post_list()
        .bind(&db).all()
        .await
        .map_err(|e| lib::emsg(e, "Post list find"))?;
    let posts: Vec<PostListItem> = posts
        .into_iter()
        .map(|data| {
            let hero = match (data.image_id, data.alt) {
                (Some(id), Some(alt)) => Some(ImgData { id, alt }),
                _ => None,
            };
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
