use super::PostError;
use crate::form::FormFooter;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn PostDeleteForm(id: String, slug: Signal<String>) -> impl IntoView {
    let post_delete = ServerAction::<PostDelete>::new();
    let pending = post_delete.pending();
    let value = post_delete.value();

    Effect::new(move |_| {
        let v = value();
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
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let found_post = prisma_client
        .post()
        .find_unique(db::post::id::equals(id.clone()))
        .select(db::post::select!({
            id
            images: select {
                id
            }
        }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;

    if found_post.is_none() {
        crate::server::serverr_404();
        return Ok(Err(PostError::NotFound));
    }
    let found_post = found_post.unwrap();
    let images_ids: Vec<String> = found_post.images.iter().map(|img| img.id.clone()).collect();

    let _images_delete_result = prisma_client
        .image()
        .delete_many(vec![db::image::id::in_vec(images_ids.clone())])
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Image delete"))?;

    prisma_client
        .post()
        .delete(db::post::id::equals(id))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post delete"))?;

    for id in images_ids {
        crate::post::delete_image_on_server(&id);
    }

    Ok(Ok(()))
}
