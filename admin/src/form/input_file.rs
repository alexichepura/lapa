use leptos::{either::Either, html, prelude::*, text_prop::TextProp};

#[component]
pub fn FileField(
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    #[prop(optional, into)] value: Option<Signal<String>>,
) -> impl IntoView {
    let inner = html::input()
        .attr("name", name.map(|v| move || v.get()))
        .attr("type", "file")
        .attr("value", value)
        .autocomplete("off");

    let label = match label {
        Some(label) => Either::Left(view! { <span>{label.get().into_owned()}</span> }),
        None => Either::Right(()),
    };

    view! { <label>{label} {inner}</label> }
}
