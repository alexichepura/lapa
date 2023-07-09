use leptos::{html::Dialog, *};
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{
    form::Input,
    image::{img_url_large, img_url_small, srcset_large, srcset_small, ImageLoadError},
    post::ImageUpload,
    util::{Loading, Pending, ResultAlert},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostImageData {
    pub id: String,
    pub alt: String,
}

#[component]
pub fn PostImages(cx: Scope, post_id: String) -> impl IntoView {
    let post_id_clone = post_id.clone();

    let image_delete = create_server_action::<ImageDelete>(cx);
    let image_upload = create_server_action::<ImageUpload>(cx);
    let image_update = create_server_action::<ImageUpdate>(cx);

    let images = create_blocking_resource(
        cx,
        move || {
            (
                post_id_clone.clone(),
                image_delete.version().get(),
                image_upload.version().get(),
                image_update.version().get(),
            )
        },
        move |(post_id, _, _, _)| get_images(cx, post_id),
    );

    view! { cx,
        <ImageUpload post_id image_upload/>
        <Transition fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                images
                    .read(cx)
                    .map(|images| match images {
                        Err(e) => view! { cx, <p>"error" {e.to_string()}</p> }.into_view(cx),
                        Ok(images) => {
                            if images.is_empty() {
                                view! { cx, <p>"No images were found."</p> }.into_view(cx)
                            } else {
                                view! { cx, <PostImagesView images image_delete image_update/> }.into_view(cx)
                            }
                        }
                    })
            }}
        </Transition>
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageEditData {
    id: String,
    alt: String,
}
type ImageEditSignal = Option<ImageEditData>;

#[component]
pub fn PostImagesView(
    cx: Scope,
    images: Vec<PostImageData>,
    image_delete: ImageDeleteAction,
    image_update: ImageUpdateAction,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = create_node_ref(cx);
    let (editing, set_editing) = create_signal::<ImageEditSignal>(cx, None);

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
            view! { cx, <PostImageModalForm image set_editing image_delete image_update/> }
                .into_view(cx)
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

type ImageDeleteAction = Action<ImageDelete, Result<ImageDeleteResult, ServerFnError>>;
type ImageUpdateAction = Action<ImageUpdate, Result<ImageUpdateResult, ServerFnError>>;
#[component]
pub fn PostImageModalForm(
    cx: Scope,
    image: ImageEditData,
    set_editing: WriteSignal<ImageEditSignal>,
    image_delete: ImageDeleteAction,
    image_update: ImageUpdateAction,
) -> impl IntoView {
    let value = image_update.value();
    let pending = image_update.pending();
    let delete_rw = image_delete.value();

    create_effect(cx, move |_| {
        if let Some(_delete_value) = delete_rw.get() {
            set_editing(None);
        };
    });

    let id_delete = image.id.clone();
    let on_delete = move |_| {
        image_delete.dispatch(ImageDelete {
            id: id_delete.clone(),
        })
    };

    view! { cx,
        <img src=img_url_large(&image.id) srcset=srcset_large(&image.id) width=500/>
        <div>
            <button on:click=on_delete>"Delete"</button>
            <hr/>
            <ActionForm action=image_update>
                <fieldset disabled=move || pending()>
                    <input type="hidden" name="id" value=image.id.clone()/>
                    <Input name="alt" label="Alt" value=image.alt.clone()/>
                    <footer>
                        <input type="submit" value="Update"/>
                        <Pending pending/>
                        <Suspense fallback=|| ()>
                            {move || match value() {
                                None => ().into_view(cx),
                                Some(v) => {
                                    let image_update_result = v
                                        .map_err(|_| ImageLoadError::ServerError)
                                        .flatten();
                                    view! { cx, <ResultAlert result=image_update_result/> }.into_view(cx)
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
    set_editing: WriteSignal<ImageEditSignal>,
) -> impl IntoView {
    let id = image.id.clone();
    let alt_clone = image.alt.clone();
    let src = img_url_small(&id);
    let srcset = srcset_small(&id);

    let on_edit = move |_| {
        set_editing(Some(ImageEditData {
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
        .find_many(vec![db::image::post_id::equals(post_id)])
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

type ImageDeleteResult = Result<(), ImageLoadError>;

#[server(ImageDelete, "/api")]
pub async fn delete_image(cx: Scope, id: String) -> Result<ImageDeleteResult, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let found_image = prisma_client
        .image()
        .find_unique(db::image::id::equals(id.clone()))
        .select(db::image::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    if found_image.is_none() {
        crate::err::serverr_404(cx);
        return Ok(Err(ImageLoadError::NotFound));
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
        .delete(db::image::id::equals(id))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
}

type ImageUpdateResult = Result<(), ImageLoadError>;
#[server(ImageUpdate, "/api")]
pub async fn image_update_alt(
    cx: Scope,
    id: String,
    alt: String,
) -> Result<ImageUpdateResult, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let found_image = prisma_client
        .image()
        .find_unique(db::image::id::equals(id.clone()))
        .select(db::image::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    if found_image.is_none() {
        crate::err::serverr_404(cx);
        return Ok(Err(ImageLoadError::NotFound));
    }

    prisma_client
        .image()
        .update(db::image::id::equals(id), vec![db::image::alt::set(alt)])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
}
