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

pub fn datetime_to_string(datetime: chrono::DateTime<chrono::FixedOffset>) -> String {
    datetime.format("%Y-%m-%d %H:%M").to_string()
}
