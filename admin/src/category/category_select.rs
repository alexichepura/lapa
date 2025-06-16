use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

use crate::util::AlertDanger;

#[component]
pub fn CategorySelect() -> impl IntoView {
    let category_resource = Resource::new_blocking(|| (), |_| get_categories());
    view! {
        <Suspense>
            <label>
                <div>Category</div>
                <select name="category_id" autocomplete="off" required>
                    {move || Suspend::new(async move {
                        match category_resource.await {
                            Ok(categories) => {
                                Either::Left(
                                    categories
                                        .into_iter()
                                        .map(|category| {
                                            view! { <option value=category.id>{category.name}</option> }
                                        })
                                        .collect_view(),
                                )
                            }
                            Err(e) => Either::Right(view! { <AlertDanger text=e.to_string() /> }),
                        }
                    })}
                </select>
            </label>
        </Suspense>
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategorySelectData {
    pub id: String,
    pub name: String,
}
#[server(GetCategorySelect, "/api")]
async fn get_categories() -> Result<Vec<CategorySelectData>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let categories = clorinde::queries::admin_post_category::list()
        .bind(&db)
        .map(|data| CategorySelectData {
            id: data.id.into(),
            name: data.name.into(),
        })
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Post category select"))?;
    Ok(categories)
}
