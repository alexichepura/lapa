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
                    <Input name="name_en" label="Name en" value=form.name />
                </div>
                <FormFooter action=category_update submit_text="Update category" />
            </fieldset>
        </ActionForm>
    }
}
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryUpdateResponse {}
#[server(CategoryUpdate, "/api")]
pub async fn category_update(
    id: String,
    slug: String,
    name_en: String,
) -> Result<Result<CategoryUpdateResponse, CategoryError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;
    let _category = prisma_client
        .category()
        .update(
            db::category::id::equals(id),
            vec![
                db::category::name_en::set(name_en),
                db::category::slug::set(slug),
            ],
        )
        .select(db::category::select!({ slug }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Category update"))?;

    return Ok(Ok(CategoryUpdateResponse {}));
}
