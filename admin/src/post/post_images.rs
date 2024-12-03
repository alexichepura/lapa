use leptos::{either::Either, html::Dialog, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    form::FormFooter,
    image::{img_url_small, srcset_small, ImageLoadError},
    post::{
        ImageDelete, ImageDeleteAction, ImageEditData, ImageEditSignal, ImageUpdate,
        ImageUpdateAction, ImageUpload, PostImageModalForm,
    },
    util::Loading,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostImageData {
    pub id: String,
    pub alt: String,
    pub order: i32,
    pub is_hero: bool,
}

#[component]
pub fn PostImages(post_id: String) -> impl IntoView {
    let post_id_clone = post_id.clone();

    let image_delete = Resource::<ImageDelete>::new_blocking();
    let image_upload = ServerAction::<ImageUpload>::new();
    let image_update = ServerAction::<ImageUpdate>::new();
    let order_action = ServerAction::<ImagesOrderUpdate>::new();
    let hero_action = ServerAction::<ImageMakeHero>::new();

    let images = Resource::new_blocking(
        move || {
            (
                post_id_clone.clone(),
                image_delete.version().get(),
                image_upload.version().get(),
                image_update.version().get(),
                order_action.version().get(),
                hero_action.version().get(),
            )
        },
        move |(post_id, _, _, _, _, _)| get_images(post_id),
    );

    view! {
        <ImageUpload post_id image_upload />
        <Transition fallback=move || {
            view! { <Loading /> }
        }>
            {move || {
                images
                    .get()
                    .map(|images| match images {
                        Err(e) => view! { <p>error {e.to_string()}</p> }.into_view(),
                        Ok(images) => {
                            view! {
                                <PostImagesView
                                    images
                                    image_delete
                                    image_update
                                    order_action
                                    hero_action
                                />
                            }
                                .into_view()
                        }
                    })
            }}

        </Transition>
    }
}

#[component]
pub fn PostImagesView(
    images: Vec<PostImageData>,
    image_delete: ImageDeleteAction,
    image_update: ImageUpdateAction,
    order_action: ImagesOrderUpdateAction,
    hero_action: ImageMakeHeroAction,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let (editing, set_editing) = create_signal::<ImageEditSignal>(None);

    let (images_sorted, set_images_sorted) = create_signal(images);

    let on_order = move |id: String, dir: i32| {
        let il = images_sorted.get().clone();
        let from_index = il.iter().position(|item| item.id == id).unwrap();
        let to_index = from_index as i32 + dir;
        set_images_sorted.update(|mut_il| {
            let removed_item = mut_il.remove(from_index);
            mut_il.insert(to_index as usize, removed_item);
            let images = mut_il
                .into_iter()
                .enumerate()
                .map(|(i, img)| PostImageData {
                    order: i as i32,
                    ..img.clone()
                })
                .collect::<Vec<_>>();
            *mut_il = images;
        });
    };

    Effect::new(move |_| {
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
        Some(image) => Either::Left(
            view! { <PostImageModalForm image set_editing image_delete image_update /> },
        ),
        None => Either::Right(()),
    };

    let order_pending = order_action.pending();
    let hero_pending = hero_action.pending();
    let disabled = move || order_pending() || hero_pending();

    let no_images = move || match images_sorted().len() {
        0 => Either::Left(view! { <p>No images were found.</p> }),
        _ => Either::Right(()),
    };

    view! {
        <fieldset prop:disabled=disabled>
            <legend>Images</legend>
            <ActionForm action=order_action>
                <For
                    each=move || images_sorted()
                    key=|image| format!("{}:{}", image.id, image.order)
                    children=move |image: PostImageData| {
                        view! { <input type="hidden" name="ids[]" value=image.id /> }
                    }
                />

                <FormFooter action=order_action submit_text="Save order" />
            </ActionForm>
            <div class="images">
                {no_images}
                <For
                    each=move || images_sorted()
                    key=|image| format!("{}:{}", image.id, image.order)
                    children=move |image: PostImageData| {
                        let is_last = image.order + 1 == images_sorted().len() as i32;
                        let id_to_make_hero = image.id.clone();
                        let make_hero = move || {
                            hero_action
                                .dispatch(ImageMakeHero {
                                    id: id_to_make_hero.clone(),
                                });
                        };
                        view! { <PostImage image set_editing on_order is_last make_hero /> }
                    }
                />

            </div>
        </fieldset>
        <dialog class="Grid-fluid-2" node_ref=dialog_element>
            {edit_view}
        </dialog>
    }
}

#[component]
pub fn PostImage<F, H>(
    image: PostImageData,
    set_editing: WriteSignal<ImageEditSignal>,
    on_order: F,
    is_last: bool,
    make_hero: H,
) -> impl IntoView
where
    F: Fn(String, i32) + 'static + Clone,
    H: Fn() + 'static + Clone,
{
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

    let is_first = image.order == 0;

    let hero_view = match image.is_hero {
        true => Either::Left(view! { <button disabled>Hero</button> }),
        false => Either::Right(view! { <button on:click=move |_| make_hero()>Make hero</button> }),
    };

    view! {
        <figure>
            <img on:click=on_edit src=src srcset=srcset width=250 />
            <figcaption>{image.alt}</figcaption>
            <footer>
                <button
                    disabled=is_first
                    on:click={
                        let on_order = on_order.clone();
                        let id = image.id.clone();
                        move |_| on_order(id.clone(), -1)
                    }
                >

                    "<"
                </button>
                {hero_view}
                <button
                    disabled=is_last
                    on:click={
                        let on_order = on_order.clone();
                        let id = image.id.clone();
                        move |_| on_order(id.clone(), 1)
                    }
                >

                    ">"
                </button>
            </footer>
        </figure>
    }
}

#[server(GetImages, "/api")]
pub async fn get_images(post_id: String) -> Result<Vec<PostImageData>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;
    let images = prisma_client
        .image()
        .find_many(vec![db::image::post_id::equals(post_id)])
        .order_by(db::image::order::order(db::SortOrder::Asc))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Images find_many"))?;

    let images: Vec<PostImageData> = images
        .iter()
        .map(|data| PostImageData {
            id: data.id.clone(),
            alt: data.alt.clone(),
            order: data.order,
            is_hero: data.is_hero,
        })
        .collect();
    Ok(images)
}

pub type ImagesOrderUpdateResult = Result<(), ImageLoadError>;
pub type ImagesOrderUpdateAction =
    Action<ImagesOrderUpdate, Result<ImagesOrderUpdateResult, ServerFnError>>;
#[server(ImagesOrderUpdate, "/api")]
pub async fn images_order_update(
    ids: Vec<String>,
) -> Result<ImagesOrderUpdateResult, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let order_update = ids.into_iter().enumerate().map(|(i, id)| {
        prisma_client
            .image()
            .update(
                db::image::id::equals(id),
                vec![db::image::order::set(i as i32)],
            )
            .select(db::image::select!({ id order }))
    });

    let _images_updated: Vec<_> = prisma_client
        ._batch(order_update)
        .await
        .map_err(|e| lib::emsg(e, "Images update batch"))?;

    Ok(Ok(()))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageMakeHeroData {
    pub hero: String,
    pub not_hero: Option<String>,
}
pub type ImageMakeHeroResult = Result<ImageMakeHeroData, ImageLoadError>;
pub type ImageMakeHeroAction = Action<ImageMakeHero, Result<ImageMakeHeroResult, ServerFnError>>;
#[server(ImageMakeHero, "/api")]
pub async fn image_make_hero(id: String) -> Result<ImageMakeHeroResult, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let current_img = prisma_client
        .image()
        .find_unique(db::image::id::equals(id.clone()))
        .select(db::image::select!({ post_id }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Image find"))?;
    if let None = current_img {
        return Ok(Err(ImageLoadError::NotFound));
    }
    let current_img = current_img.unwrap();
    let current_hero = prisma_client
        .image()
        .find_first(vec![
            db::image::post_id::equals(current_img.post_id),
            db::image::is_hero::equals(true),
        ])
        .select(db::image::select!({ id }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Image hero find"))?;

    let data: ImageMakeHeroData = prisma_client
        ._transaction()
        .run(|prisma_client| async move {
            let not_hero = if let Some(current_hero) = current_hero {
                let not_hero = prisma_client
                    .image()
                    .update(
                        db::image::id::equals(current_hero.id),
                        vec![db::image::is_hero::set(false)],
                    )
                    .select(db::image::select!({ id is_hero }))
                    .exec()
                    .await?;
                Some(not_hero.id)
            } else {
                None
            };

            prisma_client
                .image()
                .update(
                    db::image::id::equals(id),
                    vec![db::image::is_hero::set(true)],
                )
                .select(db::image::select!({ id is_hero }))
                .exec()
                .await
                .map(|hero| ImageMakeHeroData {
                    hero: hero.id,
                    not_hero,
                })
        })
        .await
        .map_err(|e| lib::emsg(e, "Image update"))?;

    Ok(Ok(data))
}
