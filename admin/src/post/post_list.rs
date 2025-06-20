use chrono::{DateTime, FixedOffset};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    util::{datetime_to_strings, AlertDanger, DateTimeStrings, Loading},
};

#[component]
pub fn PostList() -> impl IntoView {
    let posts = Resource::new_blocking(|| (), move |_| get_posts());
    let suspended = move || Suspend::new(async move {
        match posts.await {
            Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
            Ok(posts) => Either::Right(view! { <PostListItems posts /> }),
        }
    });
    view! {
        <Title text="Posts" />
        <h1>
            <span>Posts</span>
            <small>
                <A href="/post/create">Create</A>
            </small>
        </h1>
        <ul class="Card Listing">
            <Suspense fallback=move || {
                view! { <Loading /> }
            }>{suspended}</Suspense>
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
        Some(publish_at) => datetime_to_strings(publish_at),
        None => DateTimeStrings::draft(),
    };
    let class = match post.is_published {
        true => "published",
        false => "not-published",
    };
    view! {
        <li class="PostListItem">
            <A href=format!("/post/{}", post.id)>
                <div title="Published at" class=format!("PostListItem-status {}", class)>
                    {published.local}
                </div>
                <span title="Post title">{post.h1}</span>
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
    let posts = clorinde::queries::admin_post::list()
        .bind(&db)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Post list find"))?;
    let posts: Vec<PostListItem> = posts
        .into_iter()
        .map(|data| {
            let is_published: bool = match data.publish_at {
                Some(publish_at) => chrono::Utc::now().fixed_offset() > publish_at.and_utc().fixed_offset(),
                None => false,
            };
            PostListItem {
                id: data.id,
                h1: data.h1,
                created_at: data.created_at.and_utc().fixed_offset(),
                publish_at: data.publish_at.map(|dt| dt.and_utc().fixed_offset()),
                is_published,
                // hero: data.image_id,
            }
        })
        .collect();
    Ok(posts)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostListItem {
    pub id: String,
    pub h1: String,
    pub created_at: DateTime<FixedOffset>,
    pub publish_at: Option<DateTime<FixedOffset>>,
    pub is_published: bool,
    // pub hero: Option<String>,
}
