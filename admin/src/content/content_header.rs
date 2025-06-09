use chrono::{DateTime, FixedOffset};
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

use crate::util::{datetime_to_string, datetime_to_strings};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentHeaderPost {
    pub id: String,
    pub slug: String,
}
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentHeaderCategory {
    pub id: String,
    pub slug: String,
    pub name_en: String,
    pub post: Option<ContentHeaderPost>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentHeaderData {
    pub id: String,
    pub created_at: DateTime<FixedOffset>,
    pub category: Option<ContentHeaderCategory>,
}
#[component]
pub fn ContentHeader(
    header: ContentHeaderData,
    published_at: Signal<Option<DateTime<FixedOffset>>>,
) -> impl IntoView {
    let site_url = "https://magic-networks.com";
    #[derive(Clone)]
    struct Category {
        name_en: String,
        admin_href: String,
        web_href: String,
    }
    let category = header.category.map(|category| Category {
        name_en: category.name_en,
        admin_href: format!("/category/{}", category.id),
        web_href: format!(
            "{}/{}{}",
            site_url,
            category.slug,
            match category.post {
                Some(post) => format!("/{}", post.slug),
                None => "".to_string(),
            }
        ),
    });
    let created = datetime_to_strings(header.created_at);
    let published_at_utc_string = move || match published_at() {
        Some(published_at) => Either::Left(datetime_to_string(published_at)),
        None => Either::Right(()),
    };
    let category_view = match category {
        Some(category) => {
            let Category {
                admin_href,
                web_href,
                name_en,
            } = category;

            Either::Left(view! {
                <dt>"Category: "</dt>
                <dd>
                    <a href=admin_href>{name_en}</a>
                </dd>
                <br />
                <dt>"Web: "</dt>
                <dd>
                    <a href=web_href target="_blank">
                        {web_href.clone()}
                    </a>
                </dd>
            })
        }
        None => Either::Right(()),
    };
    view! {
        <header>
            <div>
                <h1>Post</h1>
                <dl>{category_view}</dl>
            </div>
            <dl>
                <dt>ID:</dt>
                <dd>{header.id}</dd>
                <br />
                <dt>Created at <small>(Local):</small></dt>
                <dd>{created.local}</dd>
                <br />
                <dt>Created at <small>(UTC):</small></dt>
                <dd>{created.utc}</dd>
                <br />
                <dt>Published at <small>(UTC):</small></dt>
                <dd>{published_at_utc_string}</dd>
            </dl>
        </header>
    }
}
