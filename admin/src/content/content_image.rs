use leptos::{either::Either, html::Dialog, prelude::*};
use serde::{Deserialize, Serialize};

use crate::content::{ImageUploadError, ContentImageUploadAction};
use crate::form::{FormFooter, Input};

#[component]
pub fn ContentImageEditor(
    content_id: String,
    editing: ReadSignal<Option<ImageEditData>>,
    set_editing: WriteSignal<Option<ImageEditData>>,
    updater: ReadSignal<Option<Box<dyn Fn(ImageEditData)>>, LocalStorage>,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let save = move |updated_image: ImageEditData| {
        let binding = updater.read_untracked();
        let updater = binding.as_ref().unwrap();
        updater(updated_image);
        set_editing(None);
    };
    let del = move || {
        set_editing(None);
    };
    let close = move || {
        set_editing(None);
    };
    let edit_view = move || match editing() {
        Some(image) => Either::Left(
            view! { <ImageEditModalForm content_id=content_id.clone() image save del close /> },
        ),
        None => Either::Right(()),
    };
    Effect::new(move |_| {
        if let Some(_id) = editing() {
            let el = dialog_element.get().expect("<dialog> to exist");
            let _modal_result = el.show_modal();
        } else {
            let el = dialog_element.get();
            if let Some(el) = el {
                let _modal_result = el.close();
            }
        }
    });
    view! { <dialog node_ref=dialog_element>{edit_view}</dialog> }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEditData {
    pub id: String,
    pub alt: String,
    pub caption: String,
}
#[component]
pub fn ImageEditModalForm(
    content_id: String,
    image: ImageEditData,
    save: impl Fn(ImageEditData) + 'static,
    del: impl Fn() + 'static,
    close: impl Fn() + 'static,
) -> impl IntoView {
    // let image_id = RwSignal::new(image.id.clone());
    let (image_id, set_image_id) = signal(image.id);
    let alt = RwSignal::new(image.alt.clone());
    let caption = RwSignal::new(image.caption.clone());
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let updated_image = ImageEditData {
            id: image_id.get_untracked(),
            alt: alt.get_untracked(),
            caption: caption.get_untracked(),
        };
        save(updated_image);
    };
    view! {
        <div>
            <button on:click=move |_e| del()>Delete</button>
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
                close();
            }>Cancel without saving</button>
        </div>
    }
}

use crate::upload::InputImage;

#[component]
pub fn ContentImageUpload(content_id: String, set_image_id: WriteSignal<String>) -> impl IntoView {
    let image_upload = ServerAction::<ContentImageUploadAction>::new();
    let value = image_upload.value();
    Effect::new(move |_| {
        let v = value.get();
        if let Some(v) = v {
            let content_image_result = v.map_err(|_| ImageUploadError::ServerError).flatten();
            if let Ok(content_image_result) = content_image_result {
                let id = content_image_result.id;
                set_image_id(id);
            }
        }
    });

    let pending = image_upload.pending();
    let (_file_name, set_file_name) = signal(None::<String>);
    let (save_byte_vec, set_save_byte_vec) = signal(None::<Vec<u8>>);
    let (_save_file, set_save_file) = signal(None::<String>);
    let (obj_url, set_obj_url) = signal(None::<String>);

    let img_value = move || {
        Some(
            serde_json::to_string(
                    &save_byte_vec().unwrap_or_default().to_vec(),
                )
                .unwrap(),
        )
    };
    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Image upload</legend>
            <div class="Grid-fluid-2">
                <div>
                    <label>
                        <div>Select image</div>
                        <InputImage set_file_name set_save_file set_obj_url set_save_byte_vec />
                    </label>
                    <ActionForm action=image_upload>
                        <input type="hidden" name="content_id" value=content_id />
                        <input type="hidden" name="img" value=img_value />
                        <label>
                            <span>Alt</span>
                            <input name="alt" />
                        </label>
                        <FormFooter action=image_upload submit_text="Upload image" />
                    </ActionForm>
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
    view! { <div class="ImageUploadPreview">{view}</div> }
}

