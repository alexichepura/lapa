use leptos::*;

#[component]
pub fn Checkbox(
    cx: Scope,
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    #[prop(optional)] set: Option<SignalSetter<String>>,
    #[prop(optional)] value: Option<Signal<String>>,
    #[prop(optional, into)] attributes: Option<MaybeSignal<AdditionalAttributes>>,
) -> impl IntoView {
    let mut inner = html::input(cx).attr("type", "checkbox");

    if let Some(name) = name {
        inner = inner.attr("name", name.get());
    }

    if let Some(value) = value {
        inner = inner.prop("value", value);
    }

    if let Some(set) = set {
        inner = inner.on(ev::input, move |ev| {
            set(event_target_value(&ev));
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
        Some(label) => view! { cx, <span>{label.get()}</span> }.into_view(cx),
        None => view! { cx, || () }.into_view(cx),
    };

    view! { cx, <label>{inner} {label}</label> }
}
