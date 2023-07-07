use chrono::{DateTime, FixedOffset, Local, ParseResult};
use leptos::*;

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    view! { cx, <p>"Loading..."</p> }
}

#[component]
pub fn AlertDanger(cx: Scope, text: String) -> impl IntoView {
    view! { cx, <div class="Alert Danger">{text}</div> }
}
#[component]
pub fn AlertSuccess(cx: Scope) -> impl IntoView {
    view! { cx, <div class="Alert Success">"Success"</div> }
}

#[component]
pub fn ResultAlert<T, E>(cx: Scope, result: Result<T, E>) -> impl IntoView
where
    E: std::error::Error,
{
    match result {
        Ok(_) => view! { cx, <AlertSuccess/> }.into_view(cx),
        Err(e) => view! { cx, <AlertDanger text=e.to_string()/> }.into_view(cx),
    }
}

#[component]
pub fn Pending(cx: Scope, pending: ReadSignal<bool>) -> impl IntoView {
    view! { cx,
        <Show when=move || pending() fallback=|_| ()>
            <progress indeterminate></progress>
        </Show>
    }
}

pub fn datetime_to_string(datetime: DateTime<FixedOffset>) -> String {
    datetime.format("%Y-%m-%d %H:%M").to_string()
}
pub fn datetime_to_html(datetime: DateTime<FixedOffset>) -> String {
    datetime.format("%Y-%m-%dT%H:%M").to_string()
}
pub fn html_to_datetime(datetime: String) -> DateTime<FixedOffset> {
    // "2023-07-08T03:43"
    // 1996-12-19T16:39:57-08:00
    let dt = datetime + ":00+03:00";
    let res = DateTime::parse_from_rfc3339(dt.as_str()).unwrap();
    return DateTime::from(res);
}

pub struct DateTimeStrings {
    pub utc: String,
    pub local: String,
}
pub fn datetime_to_strings(datetime: DateTime<FixedOffset>) -> DateTimeStrings {
    let utc = datetime.format("%Y-%m-%d %H:%M").to_string();
    let local: DateTime<Local> = DateTime::from(datetime);
    let local: String = local.format("%Y-%m-%d %H:%M").to_string();
    DateTimeStrings { utc, local }
}

pub fn datetime_from_html_input(input: String) -> ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_str(input.as_str(), "%Y-%m-%dT%H:%M")
}
