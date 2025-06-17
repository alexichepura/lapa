use leptos::prelude::*;

use super::ContentError;

#[server(ContentJsonUpdate, "/api")]
pub async fn content_json_update(
    content_id: String,
    json: String,
) -> Result<Result<(), ContentError>, ServerFnError> {
    use content::{CdnImageFormat, CdnImageSize, SlateBlock, SlateBlocks};
    let db = crate::server::db::use_db().await?;
    let slate_model = serde_json::from_str::<SlateBlocks>(&json)
        .map_err(|e| lib::emsg(e, "Content json is not sarialisable"))?;
    let images_ids: Vec<String> = slate_model
        .into_iter()
        .filter_map(|block| {
            if let SlateBlock::Img(img) = block {
                return Some(img.id);
            } else {
                return None;
            };
        })
        .collect();
    let content = clorinde::queries::admin_content::read()
        .bind(&db, &content_id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Content find"))?;
    // let current_content = prisma_web_client
    //     .content()
    //     .find_unique(db::content::id::equals(content_id.clone()))
    //     .select(db::content::select!({
    //         content_image: select {
    //             id
    //             ext
    //         }
    //     }))
    //     .exec()
    //     .await
    //     .map_err(|e| lib::emsg(e, "Content find_unique"))?;
    let Some(current_content) = content else {
        crate::server::serverr_404();
        return Ok(Err(ContentError::NotFound));
    };
    struct ImageToDelete {
        id: String,
        ext: String,
    }
    let mut images_to_delete: Vec<ImageToDelete> = vec![];
    // for img in current_content.content_image {
    //     if !images_ids.contains(&img.id) {
    //         images_to_delete.push(ImageToDelete {
    //             id: img.id,
    //             ext: img.ext,
    //         });
    //     }
    // }

    // let media_config = crate::server::use_media_config()?;
    // for img in &images_to_delete {
    //     let path = media_config.content_upload_name_ext(&img.id, &img.ext);
    //     let upload_del_result = std::fs::remove_file(&path);
    //     if let Err(e) = upload_del_result {
    //         tracing::debug!("image upload del {path} e={e}");
    //     }
    //     for image_format in CdnImageFormat::VALUES {
    //         for image_size in CdnImageSize::VALUES {
    //             let cdn_path = format!(
    //                 "{}/{}_{}.{}",
    //                 media_config.content_cdn_path, img.id, image_size, image_format
    //             );
    //             let cdn_del_result = std::fs::remove_file(&cdn_path);
    //             if let Err(e) = cdn_del_result {
    //                 tracing::debug!("image cdn del {cdn_path} e={e}");
    //             }
    //         }
    //     }
    // }

    // if images_to_delete.len() > 0 {
    //     let ids = images_to_delete.into_iter().map(|img| img.id).collect();
    //     prisma_web_client
    //         .content_image()
    //         .delete_many(vec![db::content_image::id::in_vec(ids)])
    //         .exec()
    //         .await
    //         .map_err(|e| lib::emsg(e, "content_image delete_many"))?;
    // }

    clorinde::queries::admin_content::update()
        .bind(&db, &json, &content_id)
        .await
        .map_err(|e| lib::emsg(e, "Content json string update"))?;
    return Ok(Ok(()));
}
