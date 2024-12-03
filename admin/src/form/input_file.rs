use leptos::{html, prelude::*, text_prop::TextProp};

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
        Some(label) => view! { <span>{label.get().into_owned()}</span> }.into_view(),
        None => ().into_view(),
    };

    view! { <label>{label} {inner}</label> }
}
