use std::collections::HashMap;

use leptos::{either::Either, prelude::*};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    img::{ImgData, Thumb}, util::{AlertDanger, Loading}
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
    use crate::server::{self, Image, Post};
    let db = server::use_db()?;
    let posts = Post::all()
        .order_by(Post::FIELDS.published_at.desc())
        .paginate(10)
        .collect::<Vec<_>>(&db)
        .await
        .map_err(|e| server::anyemsg(e, "Post find all"))?;
    dbg!(&posts);
    let images: HashMap<String, Image> = server::Image::filter(
            Image::FIELDS.post_id.in_set(
                posts.iter()
                    .map(|p| p.id.to_string())
                    .collect::<Vec<String>>()
            ).and(Image::FIELDS.is_hero.eq(1))
        )
        .collect::<Vec<Image>>(&db)
        .await
        .map_err(|e| server::anyemsg(e, "Images find"))?
        .into_iter()
        .map(|image| (image.post_id.to_string(), image))
        .collect();
    dbg!(&images);

    let posts: Vec<PostListItem> = posts
        .into_iter()
        .map(|data| {
            let hero = images.get(&data.id.to_string())
                .map(|image| ImgData {
                    id: image.id.to_string(),
                    alt: image.alt.clone(),
                });
            PostListItem {
                id: data.id.to_string(),
                title: data.title,
                slug: data.slug,
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
