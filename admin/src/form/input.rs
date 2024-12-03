use leptos::{html, prelude::*, text_prop::TextProp};

#[component]
pub fn Input(
    #[prop(optional, into)] type_: Option<TextProp>,
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    // #[prop(optional)] set: Option<WriteSignal<String>>,
    #[prop(optional, into)] value: Option<Signal<String>>,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let inner = html::input()
        .attr("name", name.map(|v| move || v.get()))
        .attr("type", type_.map(|v| move || v.get()))
        .attr("value", value)
        .autocomplete("off");

    // if let Some(set) = set {
    //     inner = inner.on(ev::input, move |ev| {
    //         set(event_target_value(&ev));
    //     })
    // };

    let label = match label {
        Some(label) => view! { <div>{label.get().into_owned()}</div> }.into_view(),
        None => ().into_view(),
    };

    view! { <label>{label} {inner}</label> }
}
