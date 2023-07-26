use leptos::*;

#[component]
pub(crate) fn InputImage(
    #[allow(unused_variables)] set_file_name: WriteSignal<Option<String>>,
    #[allow(unused_variables)] set_save_file: WriteSignal<Option<String>>,
    // set_save_bytes: WriteSignal<Option<js_sys::Uint8Array>>,
    #[allow(unused_variables)] set_save_byte_vec: WriteSignal<Option<Vec<u8>>>,
    #[allow(unused_variables)] set_obj_url: WriteSignal<Option<String>>,
) -> impl IntoView {
    #[allow(unused_variables)]
    let file_input = create_node_ref::<html::Input>();
    let on_file_change = move |_ev: leptos::ev::Event| {
        #[cfg(feature = "hydrate")]
        if let Some(files) = file_input.get().map(|fi| fi.files()).flatten() {
            let file = files.get(0).unwrap();
            let object_url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
            set_obj_url(Some(object_url));

            let file_blob_promise = js_sys::Promise::resolve(&file.array_buffer());
            set_file_name(Some(file.name()));
            spawn_local(async move {
                let bytes = wasm_bindgen_futures::JsFuture::from(file_blob_promise)
                    .await
                    .unwrap();
                let byte_arr = js_sys::Uint8Array::new(&bytes);
                // set_save_bytes(Some(byte_arr));
                let byte_vec = (&byte_arr).to_vec();
                set_save_byte_vec(Some(byte_vec));
                let buffer = &byte_arr.to_vec()[..];
                // let s = serde_json::to_string(&byte_arr.to_vec()).unwrap();
                //let _ = SaveFile::from_bytes_rs(buffer);
                // let sf = SaveFile::from_bytes_rs(buffer).unwrap();
                set_save_file(Some(format!("{buffer:?}")));
            })
        }
    };

    view! { <input type="file" on:change=on_file_change node_ref=file_input autocomplete="off"/> }
}
