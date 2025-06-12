use super::CategoryError;
use form::AlertDanger;
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

#[component]
pub fn CategorySelect() -> impl IntoView {
    let category_resource = Resource::new_blocking(|| (), |_| get_categories());
    view! {
        <Suspense>
            <label>
                <div>Category</div>
                <select name="category_id" autocomplete="off">
                    {move || Suspend::new(async move {
                        match category_resource.await {
                            Ok(categories) => {
                                Either::Left(
                                    categories
                                        .unwrap()
                                        .into_iter()
                                        .map(|category| {
                                            view! {
                                                <option value=category.id>{category.name_en}</option>
                                            }
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
    pub name_en: String,
}
#[server(GetCategoriesSelect, "/api")]
async fn get_categories() -> Result<Result<Vec<CategorySelectData>, CategoryError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let categories = prisma_client
        .category()
        .find_many(vec![])
        .select(db::category::select!({
            id
            name_en
        }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Categories find"))?;

    let categories: Vec<CategorySelectData> = categories
        .iter()
        .map(|p| CategorySelectData {
            id: p.id.clone(),
            name_en: p.name_en.clone(),
        })
        .collect();

    Ok(Ok(categories))
}
