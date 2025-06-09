use gloo_utils::format::JsValueSerdeExt;
use leptos::prelude::*;

use crate::content::{ContentImageEditor, ContentLinkEditor, ImageEditData, LinkEditData};

#[component]
pub fn ContentEditor(
    content_id: String,
    content_json: String,
    set_content: WriteSignal<String>,
) -> impl IntoView {
    let editor_container_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    let (link_editing, set_link_editing) = signal::<Option<LinkEditData>>(None);
    let (link_updater, set_link_updater) = signal_local::<Option<Box<dyn Fn(LinkEditData)>>>(None);

    let (image_editing, set_image_editing) = signal::<Option<ImageEditData>>(None);
    let (image_updater, set_image_updater) =
        signal_local::<Option<Box<dyn Fn(ImageEditData)>>>(None);

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let content_json_clone = content_json.clone();
            use js_sys::JsString;
            use wasm_bindgen::JsCast;
            use wasm_bindgen::JsValue;
            leptos::logging::log!("calling out to slate::start_slate");
            let Some(_editor_contaier) = editor_container_ref.get() else {
                tracing::warn!("effecting no editor container");
                return;
            };
            request_animation_frame(move || {
                let cb =
                    wasm_bindgen::prelude::Closure::wrap(Box::new(move |model_json: JsString| {
                        set_content(model_json.into());
                    })
                        as Box<dyn FnMut(JsString)>)
                    .into_js_value();

                let link_edit = wasm_bindgen::prelude::Closure::wrap(Box::new(
                    move |link: JsValue, link_update: js_sys::Function| {
                        let link: LinkEditData = link.into_serde().unwrap();
                        tracing::debug!("link={:?}", link);
                        let this = JsValue::null();
                        let on_edit = move |link_updated: LinkEditData| {
                            let l = JsValue::from_serde(&link_updated).unwrap();
                            let _ = link_update.call1(&this, &l);
                        };
                        set_link_updater(Some(Box::new(on_edit)));
                        set_link_editing(Some(link));
                    },
                )
                    as Box<dyn FnMut(JsValue, js_sys::Function)>)
                .into_js_value();

                let image_edit = wasm_bindgen::prelude::Closure::wrap(Box::new(
                    move |image: JsValue, image_update: js_sys::Function| {
                        let image: ImageEditData = image.into_serde().unwrap();
                        tracing::debug!("link={:?}", image);
                        let this = JsValue::null();
                        let on_edit = move |image_updated: ImageEditData| {
                            let l = JsValue::from_serde(&image_updated).unwrap();
                            let _ = image_update.call1(&this, &l);
                        };
                        set_image_updater(Some(Box::new(on_edit)));
                        set_image_editing(Some(image));
                    },
                )
                    as Box<dyn FnMut(JsValue, js_sys::Function)>)
                .into_js_value();

                let set_callbacks = wasm_bindgen::prelude::Closure::wrap(Box::new(
                    move |_set_cbs: JsValue| {},
                )
                    as Box<dyn FnMut(JsValue)>)
                .into_js_value();

                crate::slate::start_slate(
                    content_json_clone,
                    cb.as_ref().unchecked_ref(),
                    link_edit.as_ref().unchecked_ref(),
                    image_edit.as_ref().unchecked_ref(),
                    set_callbacks.as_ref().unchecked_ref(),
                );
            });
        });
    };

    view! {
        <div id="app" node_ref=editor_container_ref></div>
        <ContentLinkEditor editing=link_editing set_editing=set_link_editing updater=link_updater />
        <ContentImageEditor
            content_id=content_id
            editing=image_editing
            set_editing=set_image_editing
            updater=image_updater
        />
    }
}
