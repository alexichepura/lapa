use chrono::{DateTime, FixedOffset};
use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use serde::{Deserialize, Serialize};

use crate::util::{datetime_to_strings, DateTimeStrings, Loading};

#[component]
pub fn PostList(cx: Scope) -> impl IntoView {
    let posts = create_blocking_resource(cx, || (), move |_| get_posts(cx));

    view! { cx,
        <Title text="Posts"/>
        <h1>
            <span>"Posts"</span>
            <span>" "</span>
            <small>
                <A href="/posts/new">"Create"</A>
            </small>
        </h1>
        <ul class="Card Listing">
            <Suspense fallback=move || {
                view! { cx, <Loading/> }
            }>
                {move || {
                    posts
                        .read(cx)
                        .map(|posts| match posts {
                            Err(e) => view! { cx, <p>"error" {e.to_string()}</p> }.into_view(cx),
                            Ok(posts) => {
                                if posts.is_empty() {
                                    view! { cx, <p>"No posts were found."</p> }.into_view(cx)
                                } else {
                                    posts
                                        .into_iter()
                                        .map(|post| {
                                            view! { cx,
                                                <PostListItem post/>
                                            }
                                        })
                                        .collect_view(cx)
                                }
                            }
                        })
                }}
            </Suspense>
        </ul>
    }
}

#[component]
pub fn PostListItem(cx: Scope, post: PostListItem) -> impl IntoView {
    let created = datetime_to_strings(post.created_at);

    let published = match post.published_at {
        Some(published_at) => datetime_to_strings(published_at),
        None => DateTimeStrings::draft(),
    };
    let class = match post.is_published {
        true => "published",
        false => "not-published",
    };
    view! { cx,
        <li class="PostListItem">
            <A href=format!("/posts/{}", post.id)>
                <div>{created.local}</div>
                <div class=format!("PostListItem-status {}", class)>{published.local}</div>
                <span>{&post.title}</span>
            </A>
        </li>
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope) -> Result<Vec<PostListItem>, ServerFnError> {
    let prisma_client = crate::prisma::use_prisma(cx)?;
    let posts = prisma_client
        .post()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let posts: Vec<PostListItem> = posts
        .iter()
        .map(|data| {
            let is_published: bool = match data.published_at {
                Some(published_at) => chrono::Utc::now().fixed_offset() > published_at,
                None => false,
            };
            PostListItem {
                id: data.id.clone(),
                title: data.title.clone(),
                created_at: data.created_at,
                published_at: data.published_at,
                is_published,
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
}
