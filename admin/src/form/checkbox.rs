use leptos::{either::Either, html, prelude::*, text_prop::TextProp};

#[component]
pub fn Checkbox(
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] value: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    #[prop(optional, into)] checked: Option<Signal<bool>>,
) -> impl IntoView {
    let inner = html::input()
        .attr("type", "checkbox")
        .attr("name", name.map(|v| move || v.get()))
        .attr("value", value.map(|v| move || v.get()))
        .prop("checked", checked.unwrap_or_default())
        .attr("checked", checked.map(|v| move || v.get()));

    let label = match label {
        Some(label) => Either::Left(view! { <span>{label.get().into_owned()}</span> }),
        None => Either::Right(()),
    };

    view! { <label>{inner} {label}</label> }
}
