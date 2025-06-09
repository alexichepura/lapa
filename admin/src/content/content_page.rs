use leptos::{either::EitherOf3, prelude::*};
use leptos_meta::Title;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

use crate::{
    // content::{
    //     ContentForm, ContentFormData, ContentHeader, ContentHeaderBrand, ContentHeaderCategory,
    //     ContentHeaderModel, ContentHeaderPost, ContentHtml, FaqData, FaqView, PostError,
    // },
    content::{
        ContentError, ContentForm, ContentFormData, ContentHeader, ContentHeaderCategory,
        ContentHeaderPost, ContentHtml,
    },
    util::{AlertDanger, Loading},
};

use super::ContentHeaderData;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct ContentParams {
    id: String,
}
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentPageData {
    pub header: ContentHeaderData,
    pub form: ContentFormData,
    pub json: String,
}

#[component]
pub fn ContentPage() -> impl IntoView {
    let params = use_params::<ContentParams>();
    // let id = move || {
    //     params.with(|q| {
    //         log!("{:?}", q);
    //         // Err(MissingParam("id")) when navigating away from page
    //         q.as_ref()
    //             .map(|q| q.id.clone())
    //             .map_err(|_| PostError::InvalidId)
    //     })
    // };
    let id = Memo::new(move |prev: Option<&Result<String, ContentError>>| {
        params.with(|q| {
            // Memo to fix Err(MissingParam("id")) when navigating away from page inside <Outlet />
            // log!("{:?}", q);
            match q {
                Ok(q) => Ok(q.id.clone()),
                Err(_) => {
                    if let Some(Ok(prev)) = prev {
                        Ok(prev.to_owned())
                    } else {
                        Err(ContentError::InvalidId)
                    }
                }
            }
        })
    });

    let content = Resource::new_blocking(
        move || id(),
        move |id| async move {
            match id {
                Err(e) => Ok(Err(e)),
                Ok(id) => get_post(id).await,
            }
        },
    );

    view! {
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match content.await {
                    Ok(Ok(content)) => EitherOf3::A(view! { <ContentPageView content=content /> }),
                    Ok(Err(e)) => EitherOf3::B(view! { <AlertDanger text=e.to_string() /> }),
                    Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
                }
            })}
        </Suspense>
    }
}
#[component]
pub fn ContentPageView(content: ContentPageData) -> impl IntoView {
    let post_rw = RwSignal::new(content.form.clone());
    let (title, set_title) = create_slice(
        post_rw,
        |state| state.title.clone(),
        |state, title| state.title = title,
    );
    let (published_at, set_published_at) = create_slice(
        post_rw,
        |state| state.published_at.clone(),
        |state, published_at| state.published_at = published_at,
    );
    let content_id = content.form.content_id.clone();
    view! {
        <Title text=move || format!("Post: {}", title()) />
        <section class="FormPage">
            <ContentHeader header=content.header published_at />
            <ContentForm content=content.form title set_title published_at set_published_at />
            <ContentHtml content_id=content_id.clone() content_json=content.json />
        // <PostImages post_id=content_id.clone() />
        // <div class="Grid-fluid-2">
        // <PostDeleteForm id=id.clone() />
        // </div>
        </section>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Result<ContentPageData, ContentError>, ServerFnError> {
    use prisma_web_client::db;
    let prisma_web_client = crate::server::use_prisma_web()?;
    db::content::select!(content_dao {
            id
            created_at
            published_at
            title
            description
            json
            category: select {
                id
                slug
                name_en
            }
            post: select {
                id
                slug
                category: select {
                    id
                    slug
                    name_en
                }
            }
    });
    let content = prisma_web_client
        .content()
        .find_unique(db::content::id::equals(id))
        .select(content_dao::select())
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Content find"))?;

    let Some(content) = content else {
        crate::server::serverr_404();
        return Ok(Err(ContentError::NotFound));
    };
    let category = match content.category {
        Some(category) => Some(ContentHeaderCategory {
            id: category.id,
            slug: category.slug,
            name_en: category.name_en,
            post: None,
        }),
        None => content.post.map(|post| ContentHeaderCategory {
            id: post.category.id,
            slug: post.category.slug,
            name_en: post.category.name_en,
            post: match post.slug == "" {
                true => None,
                false => Some(ContentHeaderPost {
                    id: post.id,
                    slug: post.slug,
                }),
            },
        }),
    };
    let post_page = ContentPageData {
        header: ContentHeaderData {
            id: content.id.clone(),
            created_at: content.created_at,
            category,
        },
        form: ContentFormData {
            content_id: content.id,
            published_at: content.published_at,
            title: content.title,
            description: content.description,
        },
        json: content.json,
    };
    Ok(Ok(post_page))
}
