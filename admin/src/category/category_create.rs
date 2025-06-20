use crate::form::{FormFooter, Input};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use super::CategoryError;

#[component]
pub fn CategoryNew() -> impl IntoView {
    let category_create = ServerAction::<CategoryCreate>::new();
    let value = category_create.value();
    let pending = category_create.pending();
    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let create_result = v.map_err(|_| CategoryError::ServerError).flatten();
            if let Ok(id) = create_result {
                tracing::info!("navigate post category create_result ok");
                let navigate = use_navigate();
                let to = format!("/post-category/{}", id);
                navigate(&to, Default::default());
            }
        }
    });
    view! {
        <Title text="New category" />
        <h1>"New category"</h1>
        <ActionForm action=category_create>
            <fieldset prop:disabled=move || pending()>
                <legend>Data</legend>
                <div class="Grid-fluid-2">
                    <Input name="slug" label="Slug" />
                    <Input name="name" label="Name" />
                </div>
                <FormFooter action=category_create submit_text="Create category" />
            </fieldset>
        </ActionForm>
    }
    .into_any()
}

#[server(CategoryCreate, "/api")]
pub async fn category_create(
    slug: String,
    name: String,
) -> Result<Result<String, CategoryError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let id = cuid2::create_id();
    let _post_category_created_at = clorinde::queries::admin_post_category::create()
    .bind(
        &db,
        &id,
        &slug,
        &name,
        &"",
        &""
    )
    .one()
    .await
    .map_err(|e| lib::emsg(e, "Post category create"))?;
    return Ok(Ok(id));
}
