use leptos::prelude::*;

stylance::import_crate_style!(style, "src/util.module.css");

#[component]
pub fn Loading() -> impl IntoView {
    view! { <p>Loading...</p> }
}

#[component]
pub fn AlertDanger(text: String) -> impl IntoView {
    view! { <p class=style::alertDanger>{text}</p> }
}
#[component]
pub fn AlertSuccess() -> impl IntoView {
    view! { <p class=style::alertSuccess>Success</p> }
}

#[component]
pub fn ParagraphsByMultiline(text: String) -> impl IntoView {
    let lines = text.lines();
    lines
        .map(|line| {
            view! {
                <p>{line.to_string()}</p>
            }
        })
        .collect_view()
}
