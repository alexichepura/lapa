use leptos::{either::Either, prelude::*, server_fn::Http, text_prop::TextProp};
use serde::{Serialize, de::DeserializeOwned};
use server_fn::{ServerFn, codec::PostUrl};

use crate::upload::ImageUploadError;
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
pub fn ResultAlert<T: 'static, E>(result: Result<T, E>) -> impl IntoView
where
    E: std::error::Error + 'static,
{
    match result {
        Ok(_) => Either::Left(view! { <AlertSuccess /> }),
        Err(e) => Either::Right(view! { <AlertDanger text=e.to_string() /> }),
    }
}

/// Footer for forms that are submitted manually (e.g. multipart file uploads
/// dispatched as `FormData`) rather than through an `<ActionForm>`.
///
/// The `value` signal mirrors what a [`ServerAction`] would hold: the outer
/// result is the server-fn call, the inner result the upload outcome.
#[component]
pub fn UploadFooter<T, E>(
    pending: Signal<bool>,
    value: Signal<Option<Result<Result<T, ImageUploadError>, E>>>,
    #[prop(optional, into)] submit_text: Option<TextProp>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    E: std::fmt::Debug + Clone + Send + Sync + 'static,
{
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
                if pending.get() {
                    return view! { <progress></progress> }.into_any();
                }
                match value.get() {
                    None => ().into_any(),
                    Some(Ok(inner)) => view! { <ResultAlert result=inner /> }.into_any(),
                    Some(Err(e)) => view! { <AlertDanger text=format!("{e:?}") /> }.into_any(),
                }
            }}
        </footer>
    }
}

#[component]
pub fn FormFooter<ServFn, OutputProtocol, O, E>(
    // first result for 5xx, second for 4xx
    action: ServerAction<ServFn>,
    #[prop(optional, into)] submit_text: Option<TextProp>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
) -> impl IntoView
where
    ServFn: DeserializeOwned
        + ServerFn<Protocol = Http<PostUrl, OutputProtocol>, Output = Result<O, E>>
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
                    return view! { <progress></progress> }.into_any();
                }
                match value.get() {
                    None => ().into_any(),
                    Some(result) => {
                        {
                            match result {
                                Ok(result) => Either::Left(view! { <ResultAlert result=result /> }),
                                Err(e) => {
                                    Either::Right(view! { <AlertDanger text=format!("{e:?}") /> })
                                }
                            }
                        }
                            .into_any()
                    }
                }
            }}
        </footer>
    }
}
