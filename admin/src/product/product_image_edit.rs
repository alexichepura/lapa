use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    form::{FormFooter, Input},
    image::{img_url_large, srcset_large, ImageLoadError},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEditData {
    pub id: String,
    pub alt: String,
}
pub type ImageEditSignal = Option<ImageEditData>;
#[component]
pub fn PostImageModalForm(
    image: ImageEditData,
    set_editing: WriteSignal<ImageEditSignal>,
    image_delete: ServerAction<ImageDelete>,
    image_update: ServerAction<ImageUpdate>,
) -> impl IntoView {
    let pending = image_update.pending();
    let delete_rw = image_delete.value();

    let id_effect = image.id.clone();
    Effect::new(move |old| {
        let id = id_effect.clone();
        let delete_result = delete_rw.get();
        // Some(old) to prevent initial run
        if let (Some(_delete_value), Some(old)) = (delete_result, old) {
            if id == old {
                set_editing(None);
            }
        };
        id
    });

    let id_delete = image.id.clone();
    let on_delete = move |_| {
        image_delete.dispatch(ImageDelete {
            id: id_delete.clone(),
        });
    };

    view! {
        <img src=img_url_large(&image.id) srcset=srcset_large(&image.id) width=500 />
        <div>
            <button on:click=on_delete>Delete</button>
            <hr />
            <ActionForm action=image_update>
                <fieldset prop:disabled=move || pending()>
                    <input type="hidden" name="id" value=image.id.clone() />
                    <Input name="alt" label="Alt" value=image.alt.clone() />
                    <FormFooter action=image_update submit_text="Update image data" />
                </fieldset>
            </ActionForm>
            <button on:click=move |ev| {
                ev.prevent_default();
                set_editing(None);
            }>Close</button>
        </div>
    }
}

type ImageDeleteResult = Result<(), ImageLoadError>;

#[server(ImageDelete, "/api")]
pub async fn delete_image(id: String) -> Result<ImageDeleteResult, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let deleted_count = clorinde::queries::product_image::delete_by_id()
        .bind(&db, &id)
        .await
        .map_err(|e| lib::emsg(e, "Image delete"))?;
    if deleted_count == 0 {
        crate::server::serverr_404();
        return Ok(Err(ImageLoadError::NotFound));
    }
    delete_image_on_server(&id);
    Ok(Ok(()))
}

#[cfg(feature = "ssr")]
pub fn delete_image_on_server(id: &String) {
    // TODO iterate
    if let Err(e) = std::fs::remove_file(crate::image::img_path_small(&id)) {
        tracing::error!("remove_file e={e}");
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_small_retina(&id)) {
        tracing::error!("remove_file e={e}");
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_large(&id)) {
        tracing::error!("remove_file e={e}");
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_large_retina(&id)) {
        tracing::error!("remove_file e={e}");
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_upload_ext(&id, &"jpg".to_string()))
    {
        tracing::error!("remove_file e={e}");
    };
}

type ImageUpdateResult = Result<(), ImageLoadError>;
#[server(ImageUpdate, "/api")]
pub async fn image_update_alt(id: String, alt: String) -> Result<ImageUpdateResult, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let updated_count = clorinde::queries::product_image::update_alt()
        .bind(&db, &alt, &id)
        .await
        .map_err(|e| lib::emsg(e, "Image alt update"))?;
    if updated_count == 0 {
        crate::server::serverr_404();
        return Ok(Err(ImageLoadError::NotFound));
    }
    Ok(Ok(()))
}
