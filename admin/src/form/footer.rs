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
    action: Action<I, Result<Result<O, E>, ServerFnError>>, // first result for 500, second for 400
) -> impl IntoView
where
    I: Clone + ServerFn + 'static,
    O: Clone + Serialize + DeserializeOwned + 'static,
    E: Clone + Serialize + DeserializeOwned + std::error::Error + 'static,
{
    let value = action.value();
    let pending = action.pending();
    view! { cx,
        <footer>
            <input type="submit" value="SUBMIT"/>
            <Pending pending/>
            <Suspense fallback=|| ()>
                {move || match value() {
                    None => ().into_view(cx),
                    Some(result) => {
                        match result {
                            Ok(result) => view! { cx, <ResultAlert result/> }
                            .into_view(cx),
                            Err(e) => view! { cx, <AlertDanger text=e.to_string()/> }
                            .into_view(cx)
                        }
                    }
                }}
            </Suspense>
        </footer>
    }
}
