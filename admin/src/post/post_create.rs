use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use super::PostError;
use crate::{category::CategorySelect, 
    form::{FormFooter, Input}}
;

#[component]
pub fn PostCreate() -> impl IntoView {
    let action = ServerAction::<PostCreateAction>::new();
    let value = action.value();
    let pending = action.pending();
    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let id = v.map_err(|_| PostError::ServerError).flatten();
            if let Ok(id) = id {
                tracing::info!("navigate post_result ok");
                let navigate = use_navigate();
                let to = format!("/post/{}", id);
                navigate(&to, Default::default());
            }
        }
    });
    view! {
        <Title text=move || format!("Post create") />
        <section class="PostPage">
            <ActionForm action=action>
                <fieldset prop:disabled=move || pending()>
                    <legend>Data</legend>
                    <CategorySelect />
                    <Input name="slug" label="Slug" />
                    <Input name="title" label="Title" />
                    <Input name="description" label="Description" />
                    <FormFooter action=action submit_text="Create post draft" />
                </fieldset>
            </ActionForm>
        </section>
    }
}
#[server(PostCreateAction, "/api")]
pub async fn post_create(
    slug: String,
    title: String,
    description: String,
    category_id: String,
) -> Result<Result<String, PostError>, ServerFnError> {
    use clorinde::queries;
    let mut db = crate::server::db::use_db().await?;
    let id = cuid2::create_id();
    {
        let trx = db.transaction().await.map_err(|e| lib::emsg(e, "Post create transaction init"))?;
        let content_id = cuid2::create_id();
        queries::admin_content::create()
            .bind(
                &trx,
                &content_id,
            )
            .await
            .map_err(|e| lib::emsg(e, "Content create"))?;
        queries::admin_post::create()
            .bind(
                &trx,
                &id,
                &slug,
                &title,
                &description,
                &content_id,
            )
            .await
            .map_err(|e| lib::emsg(e, "Post create"))?;
        // use clorinde::{client::Params, queries::admin_post::CreateParams};
        // queries::admin_post::create()
        //     .params(&db, &CreateParams { id: &id, slug, meta_title: title, meta_description: description })
        //     .await
        //     .map_err(|e| lib::emsg(e, "Post create"))?;
        trx.commit().await.map_err(|e| lib::emsg(e, "Post create transaction"))?;
    };
    return Ok(Ok(id));
}
