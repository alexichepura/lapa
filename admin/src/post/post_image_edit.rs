use leptos::*;
use leptos_router::ActionForm;
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
pub type ImageDeleteAction = Action<ImageDelete, Result<ImageDeleteResult, ServerFnError>>;
pub type ImageUpdateAction = Action<ImageUpdate, Result<ImageUpdateResult, ServerFnError>>;
#[component]
pub fn PostImageModalForm(
    cx: Scope,
    image: ImageEditData,
    set_editing: WriteSignal<ImageEditSignal>,
    image_delete: ImageDeleteAction,
    image_update: ImageUpdateAction,
) -> impl IntoView {
    let pending = image_update.pending();
    let delete_rw = image_delete.value();

    let id_effect = image.id.clone();
    create_effect(cx, move |old| {
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
        })
    };

    view! { cx,
        <img src=img_url_large(&image.id) srcset=srcset_large(&image.id) width=500/>
        <div>
            <button on:click=on_delete>"Delete"</button>
            <hr/>
            <ActionForm action=image_update>
                <fieldset disabled=move || pending()>
                    <input type="hidden" name="id" value=image.id.clone()/>
                    <Input name="alt" label="Alt" value=image.alt.clone()/>
                    <FormFooter action=image_update submit_text="Update image data"/>
                </fieldset>
            </ActionForm>
            <button on:click=move |ev| {
                ev.prevent_default();
                set_editing(None);
            }>"Close"</button>
        </div>
    }
}

type ImageDeleteResult = Result<(), ImageLoadError>;

#[server(ImageDelete, "/api")]
pub async fn delete_image(cx: Scope, id: String) -> Result<ImageDeleteResult, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let found_image = prisma_client
        .image()
        .find_unique(db::image::id::equals(id.clone()))
        .select(db::image::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    if found_image.is_none() {
        crate::err::serverr_404(cx);
        return Ok(Err(ImageLoadError::NotFound));
    }

    delete_image_on_server(&id);

    prisma_client
        .image()
        .delete(db::image::id::equals(id))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
}

#[cfg(feature = "ssr")]
pub fn delete_image_on_server(id: &String) {
    // TODO iterate
    if let Err(e) = std::fs::remove_file(crate::image::img_path_small(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_small_retina(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_large(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_large_retina(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_upload_ext(&id, &"jpg".to_string()))
    {
        dbg!(e);
    };
}

type ImageUpdateResult = Result<(), ImageLoadError>;
#[server(ImageUpdate, "/api")]
pub async fn image_update_alt(
    cx: Scope,
    id: String,
    alt: String,
) -> Result<ImageUpdateResult, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let found_image = prisma_client
        .image()
        .find_unique(db::image::id::equals(id.clone()))
        .select(db::image::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    if found_image.is_none() {
        crate::err::serverr_404(cx);
        return Ok(Err(ImageLoadError::NotFound));
    }

    prisma_client
        .image()
        .update(db::image::id::equals(id), vec![db::image::alt::set(alt)])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
}
