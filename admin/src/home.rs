use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

use crate::util::{AlertDanger, Loading};

stylance::import_crate_style!(style, "src/home.module.css");

#[component]
pub fn HomePage() -> impl IntoView {
    let stats_all = Resource::new_blocking(|| (), move |_| get_stats(StatsPeriod::All));
    let stats_month = Resource::new_blocking(|| (), move |_| get_stats(StatsPeriod::Month));
    let stats_hour = Resource::new_blocking(|| (), move |_| get_stats(StatsPeriod::Hour));
    view! {
        <Title text="Dashboard" />
        <div>
            <h1>Dashboard</h1>
            <hr />
            <h2>Stats</h2>
            <section class=style::stats>
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
        <table class=style::statsTable>
            <caption>{caption}</caption>
            <thead>
                <tr>
                    <th class=style::statsTablePath>Path</th>
                    <th class=style::statsTableCount>Count</th>
                </tr>
            </thead>
            <tbody>
                <For
                    each=move || list.clone()
                    key=|stat| stat.path.clone()
                    children=move |stat| {
                        view! {
                            <tr>
                                <td class=style::statsTablePath>{stat.path}</td>
                                <td class=style::statsTableCount>{stat.n}</td>
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
    pub n: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatsResult {
    pub list: Vec<StatsListItem>,
}
#[server(GetStats, "/api")]
pub async fn get_stats(period: StatsPeriod) -> Result<StatsResult, ServerFnError> {
    // Cap the number of paths shown per period; aggregation happens in SQL.
    const LIMIT: i64 = 100;
    let db = crate::server::db::use_db().await?;
    let offset = 0i64;
    let now = chrono::Utc::now().naive_utc();
    let list = match period {
        StatsPeriod::All => {
            let rows = cornucopia::queries::ssr::stats_all()
                .bind(&db, &LIMIT, &offset)
                .all()
                .await
                .map_err(|e| lib::emsg(e, "SSR stats all"))?;
            rows.into_iter()
                .map(|r| StatsListItem {
                    path: r.path,
                    n: r.n,
                })
                .collect::<Vec<_>>()
        }
        StatsPeriod::Live => since_stats(&db, now - chrono::Duration::minutes(5)).await?,
        StatsPeriod::Hour => since_stats(&db, now - chrono::Duration::hours(1)).await?,
        StatsPeriod::Day => since_stats(&db, now - chrono::Duration::days(1)).await?,
        StatsPeriod::Week => since_stats(&db, now - chrono::Duration::days(7)).await?,
        StatsPeriod::Month => since_stats(&db, now - chrono::Duration::days(30)).await?,
    };
    Ok(StatsResult { list })
}

#[cfg(feature = "ssr")]
async fn since_stats(
    db: &cornucopia::deadpool_postgres::Object,
    since: chrono::NaiveDateTime,
) -> Result<Vec<StatsListItem>, ServerFnError> {
    const LIMIT: i64 = 100;
    let offset = 0i64;
    let rows = cornucopia::queries::ssr::stats_since()
        .bind(db, &since, &LIMIT, &offset)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "SSR stats since"))?;
    Ok(rows
        .into_iter()
        .map(|r| StatsListItem {
            path: r.path,
            n: r.n,
        })
        .collect())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SsrListItem {
    pub id: String,
}
