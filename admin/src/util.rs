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

// #[component]
// pub fn ResultAlert<T>(cx: Scope, result: Result<T, PostError>) -> impl IntoView {
//     match result {
//         Ok(_) => view! { cx, <AlertSuccess/> }.into_view(cx),
//         Err(e) => view! { cx, <AlertDanger text=e.to_string()/> }.into_view(cx),
//     }
// }
