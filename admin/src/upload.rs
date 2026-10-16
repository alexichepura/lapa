use leptos::{html, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageUploadError {
    #[error("Image upload read error.")]
    Read,
    #[error("Image upload format error.")]
    Format,
}

/// A file input for image uploads.
///
/// The raw file bytes are sent as a real multipart field (see the
/// `MultipartFormData` server functions); this component only mirrors the
/// selection into a preview object URL.
#[component]
pub(crate) fn InputImage(
    #[allow(unused_variables)] set_obj_url: WriteSignal<Option<String>>,
) -> impl IntoView {
    #[allow(unused_variables)]
    let file_input = NodeRef::<html::Input>::new();
    let on_file_change = move |_ev: leptos::ev::Event| {
        #[cfg(feature = "hydrate")]
        if let Some(files) = file_input.get().map(|fi| fi.files()).flatten() {
            let file = files.get(0).unwrap();
            let object_url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
            set_obj_url(Some(object_url));
        }
    };

    view! {
        <input
            type="file"
            name="img"
            accept="image/*"
            on:change=on_file_change
            node_ref=file_input
            autocomplete="off"
        />
    }
}
