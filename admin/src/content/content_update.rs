use leptos::prelude::*;

use super::ContentError;

#[server(ContentJsonUpdate, "/api")]
pub async fn content_json_update(
    content_id: String,
    json: String,
) -> Result<Result<(), ContentError>, ServerFnError> {
    use content::{CdnImageFormat, CdnImageSize, SlateBlock, SlateBlocks};
    let mut db = crate::server::db::use_db().await?;
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
    let _current_content = clorinde::queries::admin_content::read() // TODO "exists" method?
        .bind(&db, &content_id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Content find"))?.ok_or_else(|| {
            crate::server::serverr_404();
            ContentError::NotFound
        })?;
    struct ImageToDelete {
        id: String,
        ext: String,
    }
    let content_images = clorinde::queries::admin_content_image::read_by_content()
        .bind(&db, &content_id)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Content images find"))?;
    let mut images_to_delete: Vec<ImageToDelete> = vec![];
    for img in content_images {
        if !images_ids.contains(&img.id) {
            images_to_delete.push(ImageToDelete {
                id: img.id,
                ext: img.ext,
            });
        }
    }

    let media_config = crate::server::use_image_config()?;
    for img in &images_to_delete {
        let path = media_config.content_image_upload_name_ext(&img.id, &img.ext);
        let upload_del_result = std::fs::remove_file(&path);
        if let Err(e) = upload_del_result {
            tracing::debug!("image upload del {path} e={e}");
        }
        for image_format in CdnImageFormat::VALUES {
            for image_size in CdnImageSize::VALUES {
                let cdn_path = format!(
                    "{}/{}_{}.{}",
                    media_config.content_image_convert_path(), img.id, image_size, image_format
                );
                let cdn_del_result = std::fs::remove_file(&cdn_path);
                if let Err(e) = cdn_del_result {
                    tracing::debug!("image cdn del {cdn_path} e={e}");
                }
            }
        }
    }
    {
        let trx = db.transaction().await.map_err(|e| lib::emsg(e, "Content update transaction init"))?;
        if images_to_delete.len() > 0 {
            let ids_to_del: Vec<String> = images_to_delete.into_iter().map(|img| img.id).collect();
            let _deleted = clorinde::queries::admin_content_image::delete_many_by_id()
                .bind(&trx, &ids_to_del)
                .await
                .map_err(|e| lib::emsg(e, "Content images delete"))?;
        }
        clorinde::queries::admin_content::update()
            .bind(&trx, &json, &content_id)
            .await
            .map_err(|e| lib::emsg(e, "Content json string update"))?;
        trx.commit().await.map_err(|e| lib::emsg(e, "Content update transaction"))?;
    };
    return Ok(Ok(()));
}
