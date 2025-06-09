#[cfg(feature = "hydrate")]
mod csr {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(module = "/../slate/dist/main.mjs")]
    extern "C" {
        #[wasm_bindgen(js_name = start)]
        pub fn start(
            model_json: String,
            cb: &::js_sys::Function,
            link_edit: &::js_sys::Function,
            image_edit: &::js_sys::Function,
            set_callbacks: &::js_sys::Function,
        );
    }
    pub fn start_slate(
        model_json: String,
        cb: &::js_sys::Function,
        link_edit: &::js_sys::Function,
        image_edit: &::js_sys::Function,
        set_callbacks: &::js_sys::Function,
    ) {
        start(model_json, cb, link_edit, image_edit, set_callbacks);
    }
}
#[cfg(feature = "hydrate")]
pub use csr::*;
