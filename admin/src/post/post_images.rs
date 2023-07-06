use leptos::{html::Dialog, *};
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{
    form::Input,
    image::{self, img_url_large, img_url_small, srcset_large, srcset_small, ImageError},
    post::ImageUpload,
    util::{AlertDanger, AlertSuccess, Loading},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostImageData {
    pub id: String,
    pub alt: String,
}

#[component]
pub fn PostImages(cx: Scope, post_id: String) -> impl IntoView {
    let post_id_clone = post_id.clone();

    let delete_image = create_server_action::<DeleteImage>(cx);
    let images = create_blocking_resource(
        cx,
        move || (post_id_clone.clone(), delete_image.version().get()),
        move |(post_id, _)| get_images(cx, post_id),
    );

    // let images = create_blocking_resource(
    //     cx,
    //     move || (post_id.clone()),
    //     move |post_id| get_images(cx, post_id),
    // );

    view! { cx,
        <ImageUpload post_id=post_id/>
        <Transition fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                images
                    .read(cx)
                    .map(|images| match images {
                        Err(e) => {
                            view! { cx, <p>"error" {e.to_string()}</p> }
                                .into_view(cx)
                        }
                        Ok(images) => {
                            if images.is_empty() {
                                view! { cx, <p>"No images were found."</p> }
                                    .into_view(cx)
                            } else {
                                view! { cx, <PostImagesView images delete_image/> }
                                    .into_view(cx)
                            }
                        }
                    })
            }}
        </Transition>
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditImageData {
    id: String,
    alt: String,
}
type EditImageSignal = Option<EditImageData>;

#[component]
pub fn PostImagesView(
    cx: Scope,
    images: Vec<PostImageData>,
    delete_image: Action<DeleteImage, Result<ResultDeleteImage, ServerFnError>>,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = create_node_ref(cx);
    let (editing, set_editing) = create_signal::<EditImageSignal>(cx, None);

    create_effect(cx, move |_| {
        if let Some(_id) = editing() {
            let el = dialog_element().expect("<dialog> to exist");
            let _modal_result = el.show_modal();
        } else {
            let el = dialog_element();
            if let Some(el) = el {
                let _modal_result = el.close();
            }
        }
    });

    let edit_view = move || match editing() {
        Some(image) => {
            view! { cx, <PostImageModalForm image set_editing delete_image/> }.into_view(cx)
        }
        None => ().into_view(cx),
    };

    view! { cx,
        <h2>"Images"</h2>
        <div class="images">
            <For
                each=move || images.clone()
                key=|image| image.id.clone()
                view=move |cx, image: PostImageData| {
                    view! { cx, <PostImage image set_editing/> }
                }
            />
        </div>
        <dialog class="Grid-fluid-2" node_ref=dialog_element>
            {edit_view}
        </dialog>
    }
}

#[component]
pub fn PostImageModalForm(
    cx: Scope,
    image: EditImageData,
    set_editing: WriteSignal<EditImageSignal>,
    delete_image: Action<DeleteImage, Result<ResultDeleteImage, ServerFnError>>,
) -> impl IntoView {
    let image_update_alt = create_server_action::<ImageUpdateAlt>(cx);
    let value = image_update_alt.value();
    let pending = image_update_alt.pending();
    let delete_rw = delete_image.value();

    create_effect(cx, move |_| {
        if let Some(_delete_value) = delete_rw.get() {
            set_editing(None);
        };
    });

    let id_delete = image.id.clone();
    let on_delete = move |_| {
        delete_image.dispatch(DeleteImage {
            id: id_delete.clone(),
        })
    };

    view! { cx,
        <img src=img_url_large(&image.id) srcset=srcset_large(&image.id) width=500/>
        <div>
            <button on:click=on_delete>"Delete"</button>
            <hr/>
            <ActionForm action=image_update_alt>
                <fieldset disabled=move || pending()>
                    <input type="hidden" name="id" value=image.id.clone()/>
                    <Input name="alt" label="Alt" value=image.alt.clone()/>
                    <footer>
                        <input type="submit" value="Update"/>
                        <Show when=move || pending() fallback=|_| ()>
                            <progress indeterminate></progress>
                        </Show>
                        <Suspense fallback=|| ()>
                            {move || match value() {
                                None => {
                                    view! { cx, "" }
                                        .into_view(cx)
                                }
                                Some(v) => {
                                    let post_result = v.map_err(|_| ImageError::ServerError).flatten();
                                    match post_result {
                                        Ok(_) => {
                                            view! { cx, <AlertSuccess/> }
                                                .into_view(cx)
                                        }
                                        Err(e) => {
                                            view! { cx, <AlertDanger text=e.to_string()/> }
                                                .into_view(cx)
                                        }
                                    }
                                }
                            }}
                        </Suspense>
                    </footer>
                </fieldset>
            </ActionForm>
            <button on:click=move |ev| {
                ev.prevent_default();
                set_editing(None);
            }>"Close"</button>
        </div>
    }
}
#[component]
pub fn PostImage(
    cx: Scope,
    image: PostImageData,
    set_editing: WriteSignal<EditImageSignal>,
) -> impl IntoView {
    let id = image.id.clone();
    let alt_clone = image.alt.clone();
    let src = img_url_small(&id);
    let srcset = srcset_small(&id);

    let on_edit = move |_| {
        set_editing(Some(EditImageData {
            id: id.clone(),
            alt: alt_clone.clone(),
        }));
    };
    view! { cx,
        <section on:click=on_edit>
            <img src=src srcset=srcset width=250/>
            <figcaption>{image.alt}</figcaption>
        </section>
    }
}

#[server(GetImages, "/api")]
pub async fn get_images(cx: Scope, post_id: String) -> Result<Vec<PostImageData>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;
    let images = prisma_client
        .image()
        .find_many(vec![db::image::WhereParam::PostId(
            db::read_filters::StringFilter::Equals(post_id),
        )])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let images: Vec<PostImageData> = images
        .iter()
        .map(|data| PostImageData {
            id: data.id.clone(),
            alt: data.alt.clone(),
        })
        .collect();
    Ok(images)
}

type ResultDeleteImage = Result<(), image::ImageError>;

#[server(DeleteImage, "/api")]
pub async fn delete_image(cx: Scope, id: String) -> Result<ResultDeleteImage, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let found_image = prisma_client
        .image()
        .find_unique(db::image::UniqueWhereParam::IdEquals(id.clone()))
        .select(db::image::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    if found_image.is_none() {
        crate::err::serverr_404(cx);
        return Ok(Err(image::ImageError::NotFound));
    }

    // TODO iterate
    if let Err(e) = std::fs::remove_file(crate::image::img_path_small(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_small_retina(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_large(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_large_retina(&id)) {
        dbg!(e);
    };
    if let Err(e) = std::fs::remove_file(crate::image::img_path_upload_ext(&id, &"jpg".to_string()))
    {
        dbg!(e);
    };

    prisma_client
        .image()
        .delete(db::image::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
}

type ResultImageUpdateAlt = Result<(), image::ImageError>;

#[server(ImageUpdateAlt, "/api")]
pub async fn image_update_alt(
    cx: Scope,
    id: String,
    alt: String,
) -> Result<ResultImageUpdateAlt, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let found_image = prisma_client
        .image()
        .find_unique(db::image::UniqueWhereParam::IdEquals(id.clone()))
        .select(db::image::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    if found_image.is_none() {
        crate::err::serverr_404(cx);
        return Ok(Err(image::ImageError::NotFound));
    }

    prisma_client
        .image()
        .update(
            db::image::UniqueWhereParam::IdEquals(id),
            vec![db::image::alt::set(alt)],
        )
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
}
