use leptos::{either::Either, prelude::*};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    util::{AlertDanger, Loading},
};

#[component]
pub fn PostList() -> impl IntoView {
    let posts = Resource::new_blocking(|| (), move |_| get_posts());
    let suspended = move || Suspend::new(async move {
        match posts.await {
            Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
            Ok(posts) => Either::Right(view! { <PostListView posts /> }),
        }
    });
    view! {
        <h2>Posts</h2>
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>{suspended}</Suspense>
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
    view! { <div class="ProductList">{list}</div> }
}

#[component]
pub fn PostListItem(post: PostListItem) -> impl IntoView {
    // let image_view = match post.hero {
    //     Some(image) => Either::Left(view! { <Thumb image /> }),
    //     None => Either::Right(()),
    // };
    let href = format!("/{}/{}", post.category_slug ,post.slug);
    view! {
        <A href>
            <p>{post.h1}</p>
        </A>
    }
}

#[server(GetProducts, "/api")]
pub async fn get_posts() -> Result<Vec<PostListItem>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let posts = clorinde::queries::post::list()
        .bind(&db).all()
        .await
        .map_err(|e| lib::emsg(e, "Post list find"))?;
    let posts: Vec<PostListItem> = posts
        .into_iter()
        .map(|data| {
            // let hero = match (data.image_id, data.alt) {
            //     (Some(id), Some(alt)) => Some(ImgData { id, alt }),
            //     _ => None,
            // };
            PostListItem {
                id: data.id,
                h1: data.h1,
                category_slug: data.category_slug,
                slug: data.slug,
                // hero: None,
            }
        })
        .collect();
    Ok(posts)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostListItem {
    pub id: String,
    pub h1: String,
    pub category_slug: String,
    pub slug: String,
    // pub hero: Option<ImgData>,
}
