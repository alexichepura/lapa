use leptos::{html, prelude::*, text_prop::TextProp};

#[component]
pub fn Checkbox(
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] value: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    // #[prop(optional, into)] set: Option<SignalSetter<bool>>,
    #[prop(optional, into)] checked: Option<Signal<bool>>,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let inner = html::input()
        .attr("type", "checkbox")
        .attr("name", name.map(|v| move || v.get()))
        .attr("value", value.map(|v| move || v.get()))
        .prop("checked", checked.unwrap_or_default())
        .attr("checked", checked.map(|v| move || v.get()));

    // if let Some(set) = set {
    //     inner = inner.on(ev::change, move |ev| {
    //         let val = event_target_checked(&ev);
    //         set(val);
    //     })
    // };

    let label: View = match label {
        Some(label) => view! { <span>{label.get().into_owned()}</span> }.into_view(),
        None => ().into_view(),
    };

    view! { <label>{inner} {label}</label> }
}
