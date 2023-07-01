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
