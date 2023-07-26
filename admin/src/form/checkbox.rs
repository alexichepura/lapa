use leptos::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    #[prop(optional, into)] set: Option<SignalSetter<bool>>,
    #[prop(optional, into)] checked: Option<MaybeSignal<bool>>,
    #[prop(optional, into)] attributes: Option<MaybeSignal<AdditionalAttributes>>,
) -> impl IntoView {
    let mut inner = html::input().attr("type", "checkbox");

    if let Some(name) = name {
        inner = inner.attr("name", name.get());
    }

    if let Some(checked) = checked {
        inner = inner.prop("checked", checked);
        inner = inner.attr("checked", checked);
    }

    if let Some(set) = set {
        inner = inner.on(ev::change, move |ev| {
            let val = event_target_checked(&ev);
            set(val);
        })
    };

    // see leptos Form, Html
    if let Some(attributes) = attributes {
        let attributes = attributes.get();
        for (attr_name, attr_value) in attributes.into_iter() {
            let attr_name = attr_name.to_owned();
            let attr_value = attr_value.to_owned();
            inner = inner.attr(attr_name, move || attr_value.get());
        }
    }

    let label = match label {
        Some(label) => view! { <span>{label.get()}</span> }.into_view(),
        None => ().into_view(),
    };

    view! { <label>{inner} {label}</label> }
}
