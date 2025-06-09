use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use lib::leptos::FormFooter;

use crate::{category::CategorySelect, content::ContentError};

#[component]
pub fn ContentNew() -> impl IntoView {
    let post_create = ServerAction::<PostCreate>::new();
    let value = post_create.value();
    let pending = post_create.pending();
    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let post_result = v.map_err(|_| ContentError::Server).flatten();
            if let Ok(post_result) = post_result {
                tracing::info!("navigate post_result ok");
                let navigate = use_navigate();
                let to = format!("/posts/{}", post_result);
                navigate(&to, Default::default());
            }
        }
    });
    view! {
        <ActionForm action=post_create>
            <fieldset prop:disabled=move || pending()>
                <legend>Data</legend>
                <div class="Grid-fluid-2">
                    <CategorySelect />
                </div>
                <div class="Grid-fluid-2">
                    <div>
                        <label>
                            <div>Title</div>
                            <input name="title" />
                        </label>
                        <label>
                            <div>Slug</div>
                            <input name="slug" />
                        </label>
                        <label>
                            <div>Description</div>
                            <textarea name="description"></textarea>
                        </label>
                    </div>
                </div>
                <FormFooter action=post_create submit_text="Create post unpublished" />
            </fieldset>
        </ActionForm>
    }
}

#[server(PostCreate, "/api")]
pub async fn post_create(
    published_at: Option<DateTime<FixedOffset>>,
    category_id: String,
    slug: String,
    title: String,
    description: String,
) -> Result<Result<String, ContentError>, ServerFnError> {
    use prisma_web_client::db;
    let prisma_web_client = crate::server::use_prisma_web()?;
    let content_id = prisma_web_client
        ._transaction()
        .run(|prisma_client| async move {
            let content = prisma_client
                .content()
                .create(vec![
                    db::content::published_at::set(published_at),
                    db::content::title::set(title),
                    db::content::description::set(description),
                ])
                .select(db::content::select!({ id }))
                .exec()
                .await?;
            let content_id = content.id.clone();
            let _ = prisma_client
                .post()
                .create(
                    slug,
                    db::category::id::equals(category_id),
                    db::content::id::equals(content.id),
                    vec![],
                )
                .select(db::post::select!({ id }))
                .exec()
                .await?;
            Ok(content_id) as Result<_, prisma_client_rust::QueryError>
        })
        .await
        .map_err(|e| lib::emsg(e, "Post create"))?;
    return Ok(Ok(content_id));
}
