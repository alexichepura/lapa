use leptos::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    #[prop(optional, into)] set: Option<SignalSetter<bool>>,
    #[prop(optional, into)] checked: Option<MaybeSignal<bool>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let mut inner = html::input().attr("type", "checkbox").attrs(attrs);

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

    let label: View = match label {
        Some(label) => view! { <span>{label.get().into_owned()}</span> }.into_view(),
        None => ().into_view(),
    };

    view! { <label>{inner} {label}</label> }
}
