use super::ProductError;
use crate::form::FormFooter;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn ProductDeleteForm(id: String, slug: String) -> impl IntoView {
    let action = ServerAction::<ProductDelete>::new();
    let pending = action.pending();
    let value = action.value();

    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let product_result = v.map_err(|_| ProductError::ServerError).flatten();
            if let Ok(_product_result) = product_result {
                tracing::info!("navigate product_result ok");
                let navigate = use_navigate();
                let to = format!("/product");
                navigate(&to, Default::default());
            }
        }
    });

    let (input_slug, set_input_slug) = signal::<String>("".to_string());
    let disabled = Memo::new(move |_| input_slug() != slug);
    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Danger zone. Delete product.</legend>
            <ActionForm action=action>
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
                <FormFooter action=action submit_text="Delete product" disabled />
            </ActionForm>
        </fieldset>
    }
}

#[server(ProductDelete, "/api")]
pub async fn product_delete(id: String) -> Result<Result<(), ProductError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let content_id = clorinde::queries::admin_product::read_content_id()
        .bind(&db, &id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Product read content id"))?
        .ok_or_else(|| {
            crate::server::serverr_404();
            ProductError::NotFound
        })?;
    let content_images = clorinde::queries::admin_content_image::read_by_content()
        .bind(&db, &content_id)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Content images find"))?;
    let image_config = crate::server::use_image_config()?;
    for img in &content_images {
        let path = image_config.content_image_upload_name_ext(&img.id, &img.ext);
        let upload_del_result = std::fs::remove_file(&path);
        if let Err(e) = upload_del_result {
            tracing::debug!("image upload del {path} e={e}");
        }
        for image_format in content::CdnImageFormat::VALUES {
            for image_size in content::CdnImageSize::VALUES {
                let cdn_path = format!(
                    "{}/{}_{}.{}",
                    image_config.content_image_convert_path(), img.id, image_size, image_format
                );
                let cdn_del_result = std::fs::remove_file(&cdn_path);
                if let Err(e) = cdn_del_result {
                    tracing::debug!("image cdn del {cdn_path} e={e}");
                }
            }
        }
    }
    let product_images = clorinde::queries::admin_product_image::read_by_product()
        .bind(&db, &id)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Product images ids"))?;
    for img in product_images {
        let path = image_config.product_image_upload_name_ext(&img.id, &img.ext);
        let upload_del_result = std::fs::remove_file(&path);
        if let Err(e) = upload_del_result {
            tracing::debug!("image upload del {path} e={e}");
        }
        for image_format in content::CdnImageFormat::VALUES {
            for image_size in content::CdnImageSize::VALUES {
                let cdn_path = format!(
                    "{}/{}_{}.{}",
                    image_config.product_image_convert_path(), img.id, image_size, image_format
                );
                let cdn_del_result = std::fs::remove_file(&cdn_path);
                if let Err(e) = cdn_del_result {
                    tracing::debug!("image cdn del {cdn_path} e={e}");
                }
            }
        }
    }
    // product, product images and content images should be cascade deleted
    let _deleted = clorinde::queries::admin_content::delete()
        .bind(&db, &content_id)
        .await
        .map_err(|e| lib::emsg(e, "Content delete"))?;
    // let product_images_ids: Vec<String> = product_images.into_iter().map(|img| img.id).collect();
    // {
    //     let trx = db.transaction().await.map_err(|e| lib::emsg(e, "Product delete transaction init"))?;
        // let _deleted = clorinde::queries::admin_product_image::delete_many_by_id()
        //     .bind(&trx, &product_images_ids)
        //     .await
        //     .map_err(|e| lib::emsg(e, "Product images delete"))?;
        // let _deleted = clorinde::queries::admin_product::delete()
        //     .bind(&trx, &id)
        //     .await
        //     .map_err(|e| lib::emsg(e, "Product delete"))?;
        // let _deleted = clorinde::queries::admin_content::delete()
        //     .bind(&trx, &content_id)
        //     .await
        //     .map_err(|e| lib::emsg(e, "Content delete"))?;
    //     trx.commit().await.map_err(|e| lib::emsg(e, "Product delete transaction"))?;
    // };
    Ok(Ok(()))
}
