use leptos::{either::Either, html, prelude::*, text_prop::TextProp};

// https://book.leptos.dev/view/05_forms.html?highlight=bind#simplifying-controlled-inputs-with-bind
// https://book.leptos.dev/view/03_components.html#spreading-attributes-onto-components
// https://github.com/leptos-rs/leptos/blob/main/examples/spread/src/lib.rs
// Two-way data binding PR https://github.com/leptos-rs/leptos/pull/2977

#[component]
pub fn Input(
    #[prop(optional, into)] type_: Option<TextProp>,
    #[prop(optional, into)] name: Option<TextProp>,
    #[prop(optional, into)] label: Option<TextProp>,
    #[prop(optional, into)] value: Option<Signal<String>>,
    #[prop(optional)] bind: Option<RwSignal<String>>,
) -> impl IntoView {
    let input = html::input()
        .attr("name", name.map(|v| move || v.get()))
        .attr("type", type_.map(|v| move || v.get()))
        .attr("value", value)
        .autocomplete("off");

    let input = match bind {
        Some(bind) => input.bind(leptos::attr::Value, bind).into_any(),
        _ => input.into_any(),
    };

    let label = match label {
        Some(label) => Either::Left(view! { <div>{label.get().into_owned()}</div> }),
        None => Either::Right(()),
    };

    view! { <label>{label} {input}</label> }
}
