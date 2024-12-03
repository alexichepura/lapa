use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional, into)] type_: Option<TextProp>,
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    #[prop(optional)] set: Option<WriteSignal<String>>,
    #[prop(optional, into)] value: Option<MaybeSignal<String>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let mut inner = html::input().attrs(attrs);

    if let Some(name) = name {
        inner = inner.attr("name", name.get());
    }
    if let Some(type_) = type_ {
        inner = inner.attr("type", type_.get());
    }
    if let Some(value) = value {
        match value {
            MaybeSignal::Static(value) => {
                inner = inner.attr("value", value);
            }
            MaybeSignal::Dynamic(signal) => {
                inner = inner.attr("value", value);
                inner = inner.prop("value", signal);
            }
        }
    }
    if let Some(set) = set {
        inner = inner.on(ev::input, move |ev| {
            set(event_target_value(&ev));
        })
    };

    let label = match label {
        Some(label) => view! { <div>{label.get().into_owned()}</div> }.into_view(),
        None => ().into_view(),
    };

    view! { <label>{label} {inner}</label> }
}
