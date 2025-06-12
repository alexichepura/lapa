use super::CategoryError;
use crate::form::{FormFooter, Input};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryFormData {
    pub id: String,
    pub slug: String,
    pub name: String,
}

#[component]
pub fn CategoryForm(form: CategoryFormData) -> impl IntoView {
    let category_update = ServerAction::<CategoryUpdate>::new();
    let pending = category_update.pending();
    view! {
        <ActionForm action=category_update>
            <fieldset prop:disabled=move || pending()>
                <legend>Data</legend>
                <input type="hidden" name="id" value=form.id />
                <div class="Grid-fluid-2">
                    <Input name="slug" label="Slug" value=form.slug />
                    <Input name="name" label="Name" value=form.name />
                </div>
                <FormFooter action=category_update submit_text="Update category" />
            </fieldset>
        </ActionForm>
    }
}
#[server(CategoryUpdate, "/api")]
pub async fn category_update(
    id: String,
    slug: String,
    name: String,
    // title: String,
    // description: String,
) -> Result<Result<(), CategoryError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let title = "";
    let description = "";
    clorinde::queries::admin_post_category::update()
        .bind(&db, &slug, &name, &title, &description, &id)
        .await
        .map_err(|e| lib::emsg(e, "Post category update"))?;
    return Ok(Ok(()));
}
