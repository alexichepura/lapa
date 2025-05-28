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
    // use prisma_client::db;
    // let prisma_client = crate::server::use_prisma()?;
    // let now = prisma_client_rust::chrono::Utc::now().fixed_offset();
    // let posts = prisma_client
    //     .post()
    //     .find_many(vec![db::post::published_at::lt(now)])
    //     .include(db::post::include!({
    //         images(vec![db::image::is_hero::equals(true)]).take(1): select {
    //             id alt
    //         }
    //     }))
    //     .exec()
    //     .await
    //     .map_err(|e| lib::emsg(e, "Post find_many"))?;
    use crate::server::Post;
    let db = crate::server::use_db()?;
    let posts = crate::server::Post::filter(Post::FIELDS.slug.ne(""))
        .include(Post::FIELDS.images)
        // .order_by(order_by)
        .collect::<Vec<_>>(&db)
        // .all(&db)
        .await
        .map_err(|e| crate::server::anyemsg(e, "Post find all"))?;

    // let posts = posts
    //     .collect::<Vec<_>>()
    //     .await
    //     .map_err(|e| crate::server::anyemsg(e, "Posts collect"))?;

    let posts: Vec<PostListItem> = posts
        .into_iter()
        .map(|data| {
            let images = data.images.get();
            let hero = images.first().map(|image| ImgData {
                id: image.id.to_string(),
                alt: image.alt.clone(),
            });
            // let hero = None;
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
