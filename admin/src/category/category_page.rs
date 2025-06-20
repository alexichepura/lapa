use leptos::{either::EitherOf3, prelude::*};
use leptos_meta::Title;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

use super::{CategoryFormData, CategoryHeaderData};
use crate::{category::{CategoryError, CategoryForm, CategoryHeader}, util::{AlertDanger, Loading}};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct CategoryParams {
    id: String,
}
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryPageData {
    pub header: CategoryHeaderData,
    pub form: CategoryFormData,
}

#[component]
pub fn CategoryPage() -> impl IntoView {
    let params = use_params::<CategoryParams>();
    let id = Memo::new(move |prev: Option<&Result<String, CategoryError>>| {
        params.with(|q| {
            // Memo to fix Err(MissingParam("id")) when navigating away from page inside <Outlet />
            // log!("{:?}", q);
            match q {
                Ok(q) => Ok(q.id.clone()),
                Err(_) => {
                    if let Some(Ok(prev)) = prev {
                        Ok(prev.to_owned())
                    } else {
                        Err(CategoryError::InvalidId)
                    }
                }
            }
        })
    });

    let category = Resource::new_blocking(
        move || id(),
        move |id| async move {
            match id {
                Err(e) => Ok(Err(e)),
                Ok(id) => get_category(id).await,
            }
        },
    );

    view! {
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match category.await {
                    Ok(Ok(category)) => {
                        EitherOf3::A(view! { <CategoryPageView category=category /> })
                    }
                    Ok(Err(e)) => EitherOf3::B(view! { <AlertDanger text=e.to_string() /> }),
                    Err(e) => EitherOf3::C(view! { <AlertDanger text=e.to_string() /> }),
                }
            })}
        </Suspense>
    }
    .into_any()
}
#[component]
pub fn CategoryPageView(category: CategoryPageData) -> impl IntoView {
    view! {
        <Title text=format!("Category: {}", category.form.slug) />
        <section class="PostPage">
            <CategoryHeader category=category.header />
            <CategoryForm form=category.form />
        </section>
    }
}

#[server(GetCategory, "/api")]
pub async fn get_category(
    id: String,
) -> Result<Result<CategoryPageData, CategoryError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let category = clorinde::queries::admin_post_category::page()
        .bind(&db, &id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Post category find"))?;

    let Some(category) = category else {
        crate::server::serverr_404();
        return Ok(Err(CategoryError::NotFound));
    };

    let category_page = CategoryPageData {
        header: CategoryHeaderData {
            id: category.id.clone(),
            created_at: category.created_at.and_utc().fixed_offset(),
        },
        form: CategoryFormData {
            id: category.id,
            slug: category.slug,
            name: category.name,
        },
    };
    Ok(Ok(category_page))
}
