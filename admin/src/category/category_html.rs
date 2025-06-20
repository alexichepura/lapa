use super::CategoryError;
use crate::{form::FormFooter, html::HtmlEditor};
use leptos::prelude::*;

#[component]
pub fn CategoryHtml(category_id: String, content_json: String) -> impl IntoView {
    let category_html_update = ServerAction::<CategoryContentUpdate>::new();
    let pending = category_html_update.pending();

    #[cfg(feature = "hydrate")]
    let (content, set_content) = signal(String::from(content_json.clone()));

    #[cfg(feature = "ssr")]
    let content = content_json.clone();

    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Content</legend>
            <HtmlEditor content_json set_content />
            <ActionForm action=category_html_update>
                <input type="hidden" name="id" value=category_id />
                <input type="hidden" name="content_json" value=content />
                <FormFooter action=category_html_update submit_text="Save category content" />
            </ActionForm>
        </fieldset>
    }
}

#[server(CategoryContentUpdate, "/api")]
async fn category_content_update(
    id: String,
    content_json: String,
) -> Result<Result<(), CategoryError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let post = prisma_client
        .post()
        .find_unique(db::post::id::equals(id.clone()))
        .select(db::post::select!({ id }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post find"))?;

    let Some(_post) = post else {
        crate::server::serverr_404();
        return Ok(Err(CategoryError::NotFound));
    };

    let _ = prisma_client
        .post()
        .update(
            db::post::id::equals(id),
            vec![db::post::content_json::set(content_json)],
        )
        .select(db::post::select!({ id }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Post content json string update"))?;
    return Ok(Ok(()));
}
