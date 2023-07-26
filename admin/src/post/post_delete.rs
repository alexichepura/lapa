use leptos::*;
use leptos_router::{use_navigate, ActionForm};

use crate::form::FormFooter;

use super::PostError;

#[component]
pub fn PostDeleteForm(id: String, slug: Signal<String>) -> impl IntoView {
    let post_delete = create_server_action::<PostDelete>();
    let pending = post_delete.pending();
    let value = post_delete.value();

    create_effect(move |_| {
        let v = value();
        if let Some(v) = v {
            let post_result = v.map_err(|_| PostError::ServerError).flatten();
            if let Ok(_post_result) = post_result {
                log!("navigate post_result ok");
                let navigate = use_navigate();
                let to = format!("/posts");
                request_animation_frame(move || {
                    // see use_navigate docs
                    // RAF prevents action signal update warning
                    log!("navigate request_animation_frame");
                    navigate(&to, Default::default()).expect("posts route");
                });
            }
        }
    });

    let (input_slug, set_input_slug) = create_signal::<String>("".to_string());
    let disabled = create_memo(move |_| input_slug() != slug());
    view! {
        <fieldset disabled=move || pending()>
            <legend>Danger zone. Delete post.</legend>
            <ActionForm action=post_delete>
                <input type="hidden" name="id" value=id.clone()/>
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
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

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
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    prisma_client
        .post()
        .delete(db::post::id::equals(id))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    for id in images_ids {
        crate::post::delete_image_on_server(&id);
    }

    Ok(Ok(()))
}
