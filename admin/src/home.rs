use leptos::*;
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let renders = create_blocking_resource(cx, || (), move |_| get_renders(cx));
    view! { cx,
        <Title text="Dashboard"/>
        <h1>"Dashboard"</h1>
        <div class="Card">"Hello"</div>
    }
}

// enum StatsPeriod {
//     Live,
//     Hour,
//     Day,
//     Week,
//     Month,
// }

#[server(GetSsr, "/api")]
pub async fn get_renders(cx: Scope) -> Result<Vec<SsrListItem>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;
    let renders = prisma_client
        .ssr()
        .find_many(vec![])
        .select(db::ssr::select!({ id created_at path  }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    use std::collections::HashMap;

    let hmap: HashMap<String, i32> = renders.iter().fold(HashMap::new(), |mut acc, data| {
        match acc.get(&data.path) {
            Some(count) => acc.insert(data.path.clone(), count + 1),
            None => acc.insert(data.path.clone(), 1),
        };
        acc
    });

    dbg!(hmap);

    let renders: Vec<SsrListItem> = renders
        .iter()
        .map(|data| SsrListItem {
            id: data.id.clone(),
        })
        .collect();
    Ok(renders)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SsrListItem {
    pub id: String,
}
