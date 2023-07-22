use http::status::StatusCode;
use leptos::*;
use thiserror::Error;

#[cfg(feature = "ssr")]
pub fn serverr_404(cx: Scope) {
    let res_parts = leptos_axum::ResponseParts {
        status: Some(http::StatusCode::NOT_FOUND),
        ..Default::default()
    };
    let res_options_outer = use_context::<leptos_axum::ResponseOptions>(cx);
    if let Some(res_options) = res_options_outer {
        res_options.overwrite(res_parts);
    }
}

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

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

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
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
    // Get Errors from Signal
    let errors = errors.get();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

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
                    <p>Error: {error_string}</p>
                }
            }
        />
    }
}
