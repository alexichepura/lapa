use http::status::StatusCode;
use leptos::*;
use thiserror::Error;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[cfg(feature = "ssr")]
pub fn use_response(cx: Scope) -> ResponseOptions {
    use_context::<ResponseOptions>(cx).expect("to have leptos_axum::ResponseOptions provided")
}
#[cfg(feature = "ssr")]
pub fn serverr_401(cx: Scope) {
    let response = use_response(cx);
    response.set_status(StatusCode::UNAUTHORIZED);
}
#[cfg(feature = "ssr")]
pub fn serverr_404(cx: Scope) {
    let response = use_response(cx);
    response.set_status(StatusCode::NOT_FOUND);
}
#[cfg(feature = "ssr")]
pub fn serverr_400(cx: Scope) {
    let response = use_response(cx);
    response.set_status(StatusCode::BAD_REQUEST);
}

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// #[component]
// pub fn NotFound(cx: Scope) -> impl IntoView {
//     let mut errors = Errors::default();
//     errors.insert_with_default_key(AppError::NotFound);
//     view! { cx, <ErrorTemplate outside_errors=errors/> }.into_view(cx)
// }

#[component]
pub fn ErrorTemplate(
    cx: Scope,
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(cx, e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    let errors = errors.get();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("ErrorTemplate_Errors: {errors:#?}");

    #[cfg(feature = "ssr")]
    if let Some(response) = use_context::<ResponseOptions>(cx) {
        response.set_status(errors[0].status_code());
    }

    view! { cx,
        <h1>{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
        <For
            each=move || { errors.clone().into_iter().enumerate() }
            key=|(index, _error)| *index
            view=move |cx, error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! { cx,
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}