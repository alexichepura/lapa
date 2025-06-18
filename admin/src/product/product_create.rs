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
                    <Input name="meta_title" label="Meta title" />
                    <Input name="meta_description" label="Meta description" />
                    <Input name="h1" label="H1" />
                    <FormFooter action=action submit_text="Create product draft" />
                </fieldset>
            </ActionForm>
        </section>
    }
}

#[server(ProductCreate, "/api")]
pub async fn product_create(
    slug: String,
    meta_title: String,
    meta_description: String,
    h1: String,
) -> Result<Result<String, ProductError>, ServerFnError> {
    use clorinde::queries;
    let mut db = crate::server::db::use_db().await?;
    let id = cuid2::create_id();
    {
        let trx = db.transaction().await.map_err(|e| lib::emsg(e, "Product create transaction init"))?;
        let content_id = cuid2::create_id();
        queries::admin_content::create()
            .bind(
                &trx,
                &content_id,
            )
            .await
            .map_err(|e| lib::emsg(e, "Content create"))?;
        queries::admin_product::create()
            .bind(
                &trx,
                &id,
                &slug,
                &meta_title,
                &meta_description,
                &h1,
                &content_id,
            )
            .await
            .map_err(|e| lib::emsg(e, "Product create"))?;
        trx.commit().await.map_err(|e| lib::emsg(e, "Product create transaction"))?;
    };
    Ok(Ok(id))
}
