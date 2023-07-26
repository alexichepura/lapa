use leptos::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::util::{AlertDanger, AlertSuccess};

#[component]
pub fn Pending(pending: ReadSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || pending() fallback=|| ()>
            <progress indeterminate></progress>
        </Show>
    }
}

#[component]
pub fn ResultAlert<T, E>(result: Result<T, E>) -> impl IntoView
where
    E: std::error::Error,
{
    match result {
        Ok(_) => view! { <AlertSuccess/> }.into_view(),
        Err(e) => view! { <AlertDanger text=e.to_string()/> }.into_view(),
    }
}

#[component]
pub fn FormFooter<I, O, E>(
    action: Action<I, Result<Result<O, E>, ServerFnError>>, // first result for 5xx, second for 4xx
    #[prop(optional, into)] submit_text: Option<TextProp>,
    #[prop(optional, into)] disabled: Option<MaybeSignal<bool>>,
) -> impl IntoView
where
    I: Clone + ServerFn + 'static,
    O: Clone + Serialize + DeserializeOwned + 'static,
    E: Clone + Serialize + DeserializeOwned + std::error::Error + 'static,
{
    let value = action.value();
    let pending = action.pending();

    let submit_text = match submit_text {
        Some(submit_text) => submit_text,
        None => "SUBMIT".into(),
    };
    let disabled = move || match disabled {
        Some(disabled) => disabled.get(),
        None => false,
    };

    view! {
        <footer>
            <input type="submit" value=submit_text.get() disabled=disabled/>
            {move || {
                if pending() {
                    return view!{ <progress indeterminate></progress> }.into_view();
                }
                match value() {
                    None => ().into_view(),
                    Some(result) => {
                        match result {
                            Ok(result) => view! { <ResultAlert result/> }.into_view(),
                            Err(e) => view! { <AlertDanger text=e.to_string()/> }.into_view()
                        }
                    }
                }
            }}
        </footer>
    }
}
