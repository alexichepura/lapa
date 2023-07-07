use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use serde::{Deserialize, Serialize};

use crate::util::{datetime_to_string, Loading};

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
        <ul class="Card">
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
    let created = datetime_to_string(post.created_at);
    view! { cx,
        <li>
            <A href=format!("/posts/{}", post.id)>
                <span>{created}</span>
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
        .map(|data| PostListItem {
            id: data.id.clone(),
            title: data.title.clone(),
            created_at: data.created_at,
        })
        .collect();
    Ok(posts)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostListItem {
    pub id: String,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
