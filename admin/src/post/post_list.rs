use chrono::{DateTime, FixedOffset};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    image::img_url_small,
    util::{datetime_to_strings, AlertDanger, DateTimeStrings, Loading},
};

#[component]
pub fn PostList() -> impl IntoView {
    let posts = Resource::new_blocking(|| (), move |_| get_posts());

    view! {
        <Title text="Posts" />
        <h1>
            <span>Posts</span>
            <small>
                <A href="/posts/new">Create</A>
            </small>
        </h1>
        <ul class="Card Listing">
            <Suspense fallback=move || {
                view! { <Loading /> }
            }>
                {move || Suspend::new(async move {
                    match posts.await {
                        Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
                        Ok(posts) => Either::Right(view! { <PostListItems posts /> }),
                    }
                })}
            </Suspense>
        </ul>
    }
}
#[component]
pub fn PostListItems(posts: Vec<PostListItem>) -> impl IntoView {
    if posts.is_empty() {
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
    }
}

#[component]
pub fn PostListItem(post: PostListItem) -> impl IntoView {
    let created = datetime_to_strings(post.created_at);

    let published = match post.publish_at {
        Some(published_at) => datetime_to_strings(published_at),
        None => DateTimeStrings::draft(),
    };
    let class = match post.is_published {
        true => "published",
        false => "not-published",
    };
    let hero_view = match post.hero {
        Some(id) => {
            Either::Left(view! { <img title="Post hero" src=img_url_small(&id) width="36" /> })
        }
        None => Either::Right(view! { <div title="No post hero">?</div> }),
    };
    view! {
        <li class="PostListItem">
            <A href=format!("/posts/{}", post.id)>
                <div title="Published at" class=format!("PostListItem-status {}", class)>
                    {published.local}
                </div>
                {hero_view}
                <span title="Post title">{post.title}</span>
                <div title="Created at" class="PostListItem-created">
                    {created.local}
                </div>
            </A>
        </li>
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<PostListItem>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let posts = clorinde::queries::product::admin_product_list()
        .bind(&db)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Post list find"))?;
    let posts: Vec<PostListItem> = posts
        .into_iter()
        .map(|data| {
            let is_published: bool = match data.publish_at {
                Some(published_at) => chrono::Utc::now().fixed_offset() > published_at.and_utc().fixed_offset(),
                None => false,
            };
            PostListItem {
                id: data.id,
                title: data.meta_title,
                created_at: data.created_at.and_utc().fixed_offset(),
                publish_at: data.publish_at.map(|dt| dt.and_utc().fixed_offset()),
                is_published,
                hero: data.image_id,
            }
        })
        .collect();
    Ok(posts)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostListItem {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<FixedOffset>,
    pub publish_at: Option<DateTime<FixedOffset>>,
    pub is_published: bool,
    pub hero: Option<String>,
}
