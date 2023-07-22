use leptos::*;

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    view! { cx, <p>Loading...</p> }
}

#[component]
pub fn AlertDanger(cx: Scope, text: String) -> impl IntoView {
    view! { cx, <p class="AlertDanger">{text}</p> }
}
#[component]
pub fn AlertSuccess(cx: Scope) -> impl IntoView {
    view! { cx, <p class="AlertSuccess">Success</p> }
}

#[component]
pub fn ParagraphsByMultiline(cx: Scope, text: String) -> impl IntoView {
    let lines = text.lines();
    lines
        .map(|line| {
            view! { cx,
                <p>{line.to_string()}</p>
            }
        })
        .collect_view(cx)
}
