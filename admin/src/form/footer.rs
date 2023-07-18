use leptos::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::util::{AlertDanger, AlertSuccess};

#[component]
pub fn Pending(cx: Scope, pending: ReadSignal<bool>) -> impl IntoView {
    view! { cx,
        <Show when=move || pending() fallback=|_| ()>
            <progress indeterminate></progress>
        </Show>
    }
}

#[component]
pub fn ResultAlert<T, E>(cx: Scope, result: Result<T, E>) -> impl IntoView
where
    E: std::error::Error,
{
    match result {
        Ok(_) => view! { cx, <AlertSuccess/> }.into_view(cx),
        Err(e) => view! { cx, <AlertDanger text=e.to_string()/> }.into_view(cx),
    }
}

#[component]
pub fn FormFooter<I, O, E>(
    cx: Scope,
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

    view! { cx,
        <footer>
            <input type="submit" value=submit_text.get() disabled=disabled/>
            <Pending pending/>
            <Suspense fallback=|| ()>
                {move || {
                    if pending() {
                        return ().into_view(cx);
                    }
                    match value() {
                        None => ().into_view(cx),
                        Some(result) => {
                            match result {
                                Ok(result) => view! { cx, <ResultAlert result/> }
                                .into_view(cx),
                                Err(e) => view! { cx, <AlertDanger text=e.to_string()/> }
                                .into_view(cx)
                            }
                        }
                    }
                }}
            </Suspense>
        </footer>
    }
}
