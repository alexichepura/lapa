use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use super::ProductError;
use crate::{ 
    form::{FormFooter, Input}}
;

#[component]
pub fn ProductNew() -> impl IntoView {
    let action = ServerAction::<ProductCreate>::new();
    let value = action.value();
    let pending = action.pending();
    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let id = v.map_err(|_| ProductError::ServerError).flatten();
            if let Ok(id) = id {
                tracing::info!("navigate product_result ok");
                let navigate = use_navigate();
                let to = format!("/product/{}", id);
                navigate(&to, Default::default());
            }
        }
    });
    view! {
        <Title text=move || format!("Product create") />
        <section class="PostPage">
            <ActionForm action=action>
                <fieldset prop:disabled=move || pending()>
                    <legend>Data</legend>
                    <Input name="slug" label="Slug" />
                    <Input name="title" label="Title" />
                    <Input name="description" label="Description" />
                    <FormFooter action=action submit_text="Create product draft" />
                </fieldset>
            </ActionForm>
        </section>
    }
}

#[server(ProductCreate, "/api")]
pub async fn product_create(
    slug: String,
    title: String,
    description: String,
) -> Result<Result<String, ProductError>, ServerFnError> {
    use clorinde::queries;
    let db = crate::server::db::use_db().await?;
    let id = cuid2::create_id();
    queries::admin_product::create()
        .bind(
            &db,
            &id,
            &slug,
            &title,
            &description,
        )
        .await
        .map_err(|e| lib::emsg(e, "Product create"))?;
    Ok(Ok(id))
}
