use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

use crate::util::{AlertDanger, Loading};

#[component]
pub fn HomePage() -> impl IntoView {
    let stats_all = Resource::new_blocking(|| (), move |_| get_stats(StatsPeriod::All));
    let stats_month = Resource::new_blocking(|| (), move |_| get_stats(StatsPeriod::Month));
    let stats_hour = Resource::new_blocking(|| (), move |_| get_stats(StatsPeriod::Hour));
    view! {
        <Title text="Dashboard" />
        <div class="HomePage">
            <h1>Dashboard</h1>
            <hr />
            <h2>Stats</h2>
            <section class="Stats">
                <StatsTableTransition caption="All time" resource=stats_all />
                <StatsTableTransition caption="Last month" resource=stats_month />
                <StatsTableTransition caption="Last hour" resource=stats_hour />
            </section>
        </div>
    }
}

#[component]
pub fn StatsTableTransition(resource: StatsResource, caption: &'static str) -> impl IntoView {
    view! {
        <Transition fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match resource.await {
                    Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
                    Ok(stats) => Either::Right(view! { <StatsTable caption list=stats.list /> }),
                }
            })}
        </Transition>
    }
}

#[component]
pub fn StatsTable(caption: &'static str, list: Vec<StatsListItem>) -> impl IntoView {
    view! {
        <table class="StatsTable">
            <caption>{caption}</caption>
            <thead>
                <tr>
                    <th class="StatsTable-path">Path</th>
                    <th class="StatsTable-count">Count</th>
                </tr>
            </thead>
            <tbody>
                <For
                    each=move || list.clone()
                    key=|stat| stat.path.clone()
                    children=move |stat| {
                        view! {
                            <tr>
                                <td class="StatsTable-path">{stat.path}</td>
                                <td class="StatsTable-count">{stat.n}</td>
                            </tr>
                        }
                    }
                />

            </tbody>
        </table>
    }
}

type StatsResource = Resource<Result<StatsResult, ServerFnError>>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatsPeriod {
    Live,
    Hour,
    Day,
    Week,
    Month,
    All,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatsListItem {
    pub path: String,
    pub n: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatsResult {
    pub list: Vec<StatsListItem>,
}
#[server(GetStats, "/api")]
pub async fn get_stats(period: StatsPeriod) -> Result<StatsResult, ServerFnError> {
    use prisma_client::db;
    use std::collections::HashMap;
    let prisma_client = crate::server::use_prisma()?;

    let now = chrono::Utc::now().fixed_offset();
    let last_hour = now - chrono::Duration::hours(1);
    let last_month = now - chrono::Duration::days(30);
    let wh = match period {
        StatsPeriod::Hour => vec![db::ssr::created_at::gt(last_hour)],
        StatsPeriod::Month => vec![db::ssr::created_at::gt(last_month)],
        // StatsPeriod::All => vec![session::expires::lt(Utc::now().timestamp() as i32)],
        _ => vec![],
    };

    let renders = prisma_client
        .ssr()
        .find_many(wh)
        .select(db::ssr::select!({ id created_at path  }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "SSR counter find_many"))?;

    let hmap: HashMap<String, i32> = renders.iter().fold(HashMap::new(), |mut acc, data| {
        match acc.get(&data.path) {
            Some(count) => acc.insert(data.path.clone(), count + 1),
            None => acc.insert(data.path.clone(), 1),
        };
        acc
    });

    let mut list = hmap
        .into_iter()
        .map(|(path, n)| StatsListItem { path, n })
        .collect::<Vec<StatsListItem>>();

    list.sort_by(|a, b| b.n.cmp(&a.n));

    Ok(StatsResult { list })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SsrListItem {
    pub id: String,
}
