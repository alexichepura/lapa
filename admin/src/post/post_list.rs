use chrono::{DateTime, FixedOffset};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{
    image::img_url_small,
    util::{datetime_to_strings, DateTimeStrings, Loading},
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
                {move || {
                    posts
                        .get()
                        .map(|posts| match posts {
                            Err(e) => Either::Left(view! { <p>error {e.to_string()}</p> }),
                            Ok(posts) => {
                                Either::Right(
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
                                    },
                                )
                            }
                        })
                }}

            </Suspense>
        </ul>
    }
}

#[component]
pub fn PostListItem(post: PostListItem) -> impl IntoView {
    let created = datetime_to_strings(post.created_at);

    let published = match post.published_at {
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
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;
    let posts = prisma_client
        .post()
        .find_many(vec![])
        .include(db::post::include!({
            images(vec![db::image::is_hero::equals(true)]).take(1): select {
                id
            }
        }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post find_many"))?;

    let posts: Vec<PostListItem> = posts
        .iter()
        .map(|data| {
            let is_published: bool = match data.published_at {
                Some(published_at) => chrono::Utc::now().fixed_offset() > published_at,
                None => false,
            };
            let hero = data.images.first().map(|image| image.id.clone());
            PostListItem {
                id: data.id.clone(),
                title: data.title.clone(),
                created_at: data.created_at,
                published_at: data.published_at,
                is_published,
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
    pub created_at: DateTime<FixedOffset>,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub is_published: bool,
    pub hero: Option<String>,
}
