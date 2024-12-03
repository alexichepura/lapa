use leptos::{prelude::*, text_prop::TextProp};
use serde::{de::DeserializeOwned, Serialize};
use server_fn::{codec::PostUrl, ServerFn};

use crate::util::{AlertDanger, AlertSuccess};

#[component]
pub fn Pending(pending: ReadSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || pending() fallback=|| ()>
            <progress></progress>
        </Show>
    }
}

#[component]
pub fn ResultAlert<T, E>(result: Result<T, E>) -> impl IntoView
where
    E: std::error::Error,
{
    match result {
        Ok(_) => view! { <AlertSuccess /> }.into_view(),
        Err(e) => view! { <AlertDanger text=e.to_string() /> }.into_view(),
    }
}

#[component]
pub fn FormFooter<ServFn, O, E>(
    // first result for 5xx, second for 4xx
    action: ServerAction<ServFn>,
    #[prop(optional, into)] submit_text: Option<TextProp>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
) -> impl IntoView
where
    ServFn: DeserializeOwned
        + ServerFn<InputEncoding = PostUrl, Output = Result<O, E>>
        + Clone
        + Send
        + Sync
        + 'static,
    ServFn::Output: Send + Sync + 'static + Clone,
    ServFn::Error: Send + Sync + 'static + Clone,
    O: Clone + Serialize + DeserializeOwned + 'static + Send + Sync,
    E: Clone + Serialize + DeserializeOwned + std::error::Error + 'static + Send + Sync,
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
            <input type="submit" value=submit_text.get() disabled=disabled />
            {move || {
                if pending() {
                    return view! { <progress></progress> }.into_view();
                }
                match value() {
                    None => ().into_view(),
                    Some(result) => {
                        match result {
                            Ok(result) => view! { <ResultAlert result /> }.into_view(),
                            Err(e) => view! { <AlertDanger text=e.to_string() /> }.into_view(),
                        }
                    }
                }
            }}

        </footer>
    }
}
