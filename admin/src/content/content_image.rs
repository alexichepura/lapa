use leptos::{either::Either, html::Dialog, prelude::*};
use serde::{Deserialize, Serialize};

use wasm_bindgen::JsCast;

use crate::content::{ImageResult, content_image_upload};
use crate::form::{Input, UploadFooter};
use crate::upload::{ImageUploadError, InputImage};

stylance::import_crate_style!(
    #[allow(dead_code)]
    grid,
    "src/grid.module.css"
);
stylance::import_crate_style!(
    #[allow(dead_code)]
    images,
    "src/images.module.css"
);

#[component]
pub fn ContentImageEditor(
    content_id: String,
    editing: ReadSignal<Option<ImageEditData>>,
    on_save: Callback<ImageEditData>,
    on_delete: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let edit_view = move || match editing() {
        Some(image) => Either::Left(view! {
            <ImageEditModalForm content_id=content_id.clone() image on_save on_delete on_close />
        }),
        None => Either::Right(()),
    };
    Effect::new(move |_| {
        if editing().is_some() {
            let el = dialog_element.get().expect("<dialog> to exist");
            let _modal_result = el.show_modal();
        } else {
            let el = dialog_element.get();
            if let Some(el) = el {
                el.close();
            }
        }
    });
    view! { <dialog node_ref=dialog_element>{edit_view}</dialog> }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImageEditData {
    pub id: String,
    pub alt: String,
    pub caption: String,
}

#[component]
fn ImageEditModalForm(
    content_id: String,
    image: ImageEditData,
    on_save: Callback<ImageEditData>,
    on_delete: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let (image_id, set_image_id) = signal(image.id);
    let alt = RwSignal::new(image.alt.clone());
    let caption = RwSignal::new(image.caption.clone());
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        on_save.run(ImageEditData {
            id: image_id.get_untracked(),
            alt: alt.get_untracked(),
            caption: caption.get_untracked(),
        });
    };
    view! {
        <div>
            <button
                on:click=move |ev| {
                    ev.prevent_default();
                    on_delete.run(());
                }
            >
                Delete
            </button>
            <hr />
            <ContentImageUpload content_id set_image_id />
            <hr />
            <form on:submit=on_submit>
                <fieldset>
                    <Input name="alt" label="Alt" value=image.alt bind=alt />
                    <Input name="caption" label="Caption" value=image.caption bind=caption />
                </fieldset>
                <button type="submit">Ok</button>
            </form>
            <button on:click=move |ev| {
                ev.prevent_default();
                on_close.run(());
            }>Cancel without saving</button>
        </div>
    }
}

#[component]
pub fn ContentImageUpload(content_id: String, set_image_id: WriteSignal<String>) -> impl IntoView {
    let pending = RwSignal::new(false);
    let value =
        RwSignal::<Option<Result<Result<ImageResult, ImageUploadError>, ServerFnError>>>::new(None);
    let (obj_url, set_obj_url) = signal(None::<String>);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let target = ev
            .target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlFormElement>();
        let form_data = web_sys::FormData::new_with_form(&target).unwrap();
        pending.set(true);
        leptos::task::spawn_local(async move {
            let result = content_image_upload(form_data.into()).await;
            if let Ok(Ok(image)) = &result {
                set_image_id(image.id.clone());
            }
            value.set(Some(result));
            pending.set(false);
        });
    };

    view! {
        <fieldset prop:disabled=move || pending.get()>
            <legend>Image upload</legend>
            <div class=grid::gridFluid2>
                <div>
                    <form on:submit=on_submit>
                        <label>
                            <div>Select image</div>
                            <InputImage set_obj_url />
                        </label>
                        <input type="hidden" name="content_id" value=content_id />
                        <label>
                            <span>Alt</span>
                            <input name="alt" />
                        </label>
                        <UploadFooter
                            pending=Signal::from(pending)
                            value=Signal::from(value)
                            submit_text="Upload image"
                        />
                    </form>
                </div>
                <ImageUploadPreview obj_url />
            </div>
        </fieldset>
    }
}

#[component]
pub fn ImageUploadPreview(obj_url: ReadSignal<Option<String>>) -> impl IntoView {
    let view = move || match obj_url.get() {
        Some(url) => Either::Left(view! { <img src=url /> }),
        None => Either::Right(view! { <p>Upload preview</p> }),
    };
    view! { <div class=images::imageUploadPreview>{view}</div> }
}
