use chrono::{DateTime, FixedOffset};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::util::{datetime_to_strings, AlertDanger, DateTimeStrings, Loading};

#[component]
pub fn ContentList() -> impl IntoView {
    let posts = Resource::new_blocking(|| (), move |_| get_posts());

    view! {
        <Title text="Contents" />
        <h1>
            <span>Contents</span>
            <A href="/content/new">" +1"</A>
        </h1>

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
    }
}
#[component]
pub fn PostListItems(posts: Vec<ContentListItem>) -> impl IntoView {
    if posts.is_empty() {
        Either::Left(view! { <p>"0 contents"</p> })
    } else {
        Either::Right(view! {
            <ul class="Card Listing">
                {posts
                    .into_iter()
                    .map(|post| {
                        view! { <PostListItem post /> }
                    })
                    .collect_view()}

            </ul>
        })
    }
}

#[component]
pub fn PostListItem(post: ContentListItem) -> impl IntoView {
    let created = datetime_to_strings(post.created_at);

    let published = match post.published_at {
        Some(published_at) => datetime_to_strings(published_at),
        None => DateTimeStrings::draft(),
    };
    let class = match post.is_published {
        true => "published",
        false => "not-published",
    };
    // let hero_view = match post.hero {
    //     Some(id) => Either::Left(view! { <img src=img_url_small(&id) width="36" /> }),
    //     None => Either::Right(view! { <div>?</div> }),
    // };
    let category = match post.category {
        Some(category) => {
            let post_view = match category.post {
                Some(_post) => "post",
                None => "category",
            };
            Either::Left(view! {
                <A href=format!("/category/{}", category.id)>{category.slug}</A>
                <span>{post_view}</span>
            })
        }
        None => Either::Right(view! { <AlertDanger text="Invariant".to_string() /> }),
    };
    view! {
        <li>
            <div>{created.utc}</div>
            <div class=format!("ContentListItem-status {}", class)>{published.utc}</div>
            {category}
            <A href=format!("/content/{}", post.id)>{post.title}</A>
        // {hero_view}
        </li>
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<ContentListItem>, ServerFnError> {
    let prisma_web_client = crate::server::use_prisma_web()?;
    prisma_web_client::db::content::select!(contents_dao {
            id
            created_at
            published_at
            title
            category: select {
                id
                slug
                name_en
            }
            post: select {
                slug
                category: select {
                    id
                    slug
                }
            }
    });
    let contents = prisma_web_client
        .content()
        .find_many(vec![])
        .select(contents_dao::select())
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Content find_many"))?;

    let contents: Vec<ContentListItem> = contents
        .into_iter()
        .map(|content| {
            let is_published: bool = match content.published_at {
                Some(published_at) => chrono::Utc::now().fixed_offset() > published_at,
                None => false,
            };
            let contents_dao::Data { category, post, .. } = content;
            #[derive(Default)]
            struct ContentPost {
                // slug: Option<String>,
                // category_slug: Option<String>,
                hero: Option<String>,
            }
            let content_post: ContentPost = post
                .clone()
                .map(|_post| {
                    // let hero = post.images.into_iter().next().map(|image| image.id);
                    ContentPost {
                        // slug: Some(post.slug),
                        // category_slug: Some(post.category.slug),
                        hero: None,
                    }
                })
                .unwrap_or_default();

            let category = match category {
                Some(category) => Some(ContentListItemCategory {
                    id: category.id,
                    slug: category.slug,
                    post: None,
                }),
                None => post.map(|post| ContentListItemCategory {
                    id: post.category.id,
                    slug: post.category.slug,
                    post: match post.slug == "" {
                        true => None,
                        false => Some(post.slug),
                    },
                }),
            };

            ContentListItem {
                id: content.id.clone(),
                title: content.title.clone(),
                created_at: content.created_at,
                published_at: content.published_at,
                is_published,
                hero: content_post.hero,
                category,
            }
        })
        .collect();
    Ok(contents)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentListItem {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<FixedOffset>,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub is_published: bool,
    pub hero: Option<String>,
    pub category: Option<ContentListItemCategory>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentListItemCategory {
    pub id: String,
    pub slug: String,
    pub post: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentListItemBrand {
    pub id: String,
    pub slug: String,
    pub model: Option<ContentListItemModel>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentListItemModel {
    pub id: String,
    pub slug: String,
}
