use super::PostError;
use crate::form::FormFooter;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn PostDeleteForm(
    id: String,
    #[prop(optional, into)] slug: Signal<String>,
    // slug: Signal<String>
) -> impl IntoView {
    let post_delete = ServerAction::<PostDelete>::new();
    let pending = post_delete.pending();
    let value = post_delete.value();

    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let post_result = v.map_err(|_| PostError::ServerError).flatten();
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

type PostDeleteResult = Result<(), PostError>;

#[server(PostDelete, "/api")]
pub async fn post_delete(id: String) -> Result<PostDeleteResult, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let content_id = clorinde::queries::admin_post::read_content_id()
        .bind(&db, &id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Post read content id"))?
        .ok_or_else(|| {
            crate::server::serverr_404();
            PostError::NotFound
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
    // post and content images should be cascade deleted
    let _deleted = clorinde::queries::admin_content::delete()
        .bind(&db, &content_id)
        .await
        .map_err(|e| lib::emsg(e, "Content delete"))?;
    // {
    //     let trx = db.transaction().await.map_err(|e| lib::emsg(e, "Post delete transaction init"))?;
    //     let _deleted = clorinde::queries::admin_post::delete()
    //         .bind(&trx, &id)
    //         .await
    //         .map_err(|e| lib::emsg(e, "Post delete"))?;
    //     trx.commit().await.map_err(|e| lib::emsg(e, "Post delete transaction"))?;
    // };
    Ok(Ok(()))
}
