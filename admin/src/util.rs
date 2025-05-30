use chrono::{DateTime, Duration, FixedOffset, Local, ParseResult};
use leptos::prelude::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! { <p>Loading...</p> }
}

#[component]
pub fn AlertDanger(text: String) -> impl IntoView {
    view! { <div class="Alert Danger">{text}</div> }
}
#[component]
pub fn AlertSuccess() -> impl IntoView {
    view! { <div class="Alert Success">Success</div> }
}

pub fn datetime_to_string(datetime: DateTime<FixedOffset>) -> String {
    datetime.format("%Y-%m-%d %H:%M").to_string()
}
// pub fn datetime_to_html(datetime: DateTime<FixedOffset>) -> String {
//     datetime.format("%Y-%m-%dT%H:%M").to_string()
// }
pub fn datetime_to_local_html(datetime: DateTime<FixedOffset>) -> String {
    let local: DateTime<Local> = DateTime::from(datetime);
    local.format("%Y-%m-%dT%H:%M").to_string()
}
pub fn html_local_to_datetime(datetime: &String) -> DateTime<FixedOffset> {
    // "2023-07-08T03:43" to "2023-07-08T03:43:00+03:00"
    let dt = format!("{}:00+00:00", datetime);
    let fixed = DateTime::parse_from_rfc3339(dt.as_str()).unwrap();
    let local = Local::now();
    let offset = local.offset();
    let offset_seconds: i32 = offset.local_minus_utc();
    let fixed = fixed - Duration::seconds(offset_seconds as i64);
    return fixed;
}

pub struct DateTimeStrings {
    pub utc: String,
    pub local: String,
}
impl DateTimeStrings {
    pub fn draft() -> Self {
        Self {
            local: "draft".to_string(),
            utc: "draft".to_string(),
        }
    }
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
