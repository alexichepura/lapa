use super::ProductError;
use crate::form::FormFooter;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn ProductDeleteForm(id: String, slug: Signal<String>) -> impl IntoView {
    let post_delete = ServerAction::<PostDelete>::new();
    let pending = post_delete.pending();
    let value = post_delete.value();

    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let post_result = v.map_err(|_| ProductError::ServerError).flatten();
            if let Ok(_post_result) = post_result {
                tracing::info!("navigate post_result ok");
                let navigate = use_navigate();
                let to = format!("/posts");
                navigate(&to, Default::default());
            }
        }
    });

    let (input_slug, set_input_slug) = signal::<String>("".to_string());
    let disabled = Memo::new(move |_| input_slug() != slug());
    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Danger zone. Delete post.</legend>
            <ActionForm action=post_delete>
                <input type="hidden" name="id" value=id.clone() />
                <label>
                    <div>Slug</div>
                    <input
                        value=input_slug
                        prop:value=input_slug
                        on:input=move |ev| {
                            set_input_slug(event_target_value(&ev));
                        }
                    />

                </label>
                <FormFooter action=post_delete submit_text="Delete post" disabled />
            </ActionForm>
        </fieldset>
    }
}

type PostDeleteResult = Result<(), ProductError>;

#[server(PostDelete, "/api")]
pub async fn product_delete(id: String) -> Result<PostDeleteResult, ServerFnError> {
    let mut db = crate::server::db::use_db().await?;
    let exists = clorinde::queries::admin_product::by_id_check()
        .bind(&db, &id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Product by id check"))?
        .is_some();
    if !exists {
        crate::server::serverr_404();
        return Ok(Err(ProductError::NotFound));
    }
    let images_ids = clorinde::queries::admin_product::images_ids()
        .bind(&db, &id)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Product images ids"))?;

    {
        let trx = db.transaction().await.map_err(|e| lib::emsg(e, "Product delete transaction init"))?;
        let _deleted = clorinde::queries::product_image::delete_many_by_id()
            .bind(&trx, &images_ids)
            .await
            .map_err(|e| lib::emsg(e, "Product images delete"))?;
        let _deleted = clorinde::queries::admin_product::delete()
            .bind(&trx, &id)
            .await
            .map_err(|e| lib::emsg(e, "Product delete"))?;
        trx.commit().await.map_err(|e| lib::emsg(e, "Product delete transaction"))?;
    };
    for id in images_ids {
        crate::product::delete_image_on_server(&id);
    }
    Ok(Ok(()))
}
