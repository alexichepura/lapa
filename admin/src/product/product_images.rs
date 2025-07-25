use leptos::{either::Either, html::Dialog, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    err::AppError, form::FormFooter, product::{
        ImageDelete, ImageEditData, ImageEditSignal, ImageUpdate, ImageUpload, PostImageModalForm,
    }, util::{AlertDanger, Loading}
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductImageData {
    pub id: String,
    pub alt: String,
    pub order: i32,
    pub is_hero: bool,
}

#[component]
pub fn ProductImages(product_id: String) -> impl IntoView {
    let post_id_clone = product_id.clone();

    let image_delete = ServerAction::<ImageDelete>::new();
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
        <ImageUpload product_id image_upload />
        <Transition fallback=move || {
            view! { <Loading /> }
        }>
            {move || Suspend::new(async move {
                match images.await {
                    Err(e) => Either::Left(view! { <AlertDanger text=e.to_string() /> }),
                    Ok(images) => {
                        Either::Right(
                            view! {
                                <ProductImagesView
                                    images
                                    image_delete
                                    image_update
                                    order_action
                                    hero_action
                                />
                            },
                        )
                    }
                }
            })}
        </Transition>
    }
}

#[component]
fn ProductImagesView(
    images: Vec<ProductImageData>,
    image_delete: ServerAction<ImageDelete>,
    image_update: ServerAction<ImageUpdate>,
    order_action: ServerAction<ImagesOrderUpdate>,
    hero_action: ServerAction<ImageMakeHero>,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let (editing, set_editing) = signal::<ImageEditSignal>(None);

    let (images_sorted, set_images_sorted) = signal(images);

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
                .map(|(i, img)| ProductImageData {
                    order: i as i32,
                    ..img.clone()
                })
                .collect::<Vec<_>>();
            *mut_il = images;
        });
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

    let children = move |image: ProductImageData| {
        let is_last = image.order + 1 == images_sorted().len() as i32;
        let id_to_make_hero = image.id.clone();
        let make_hero = move || {
            hero_action
                .dispatch(ImageMakeHero {
                    id: id_to_make_hero.clone(),
                });
        };
        view! { <ProductImage image set_editing on_order is_last make_hero /> }
    };
    view! {
        <fieldset prop:disabled=disabled>
            <legend>Images</legend>
            <ActionForm action=order_action>
                <For
                    each=move || images_sorted()
                    key=|image| format!("{}:{}", image.id, image.order)
                    children=move |image: ProductImageData| {
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
                    children=children
                />

            </div>
        </fieldset>
        <dialog class="Grid-fluid-2" node_ref=dialog_element>
            {edit_view}
        </dialog>
    }
}

#[component]
pub fn ProductImage<F, H>(
    image: ProductImageData,
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
    let src = format!("/product-image/{}", &id);

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
            <img on:click=on_edit src=src width=250 />
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
pub async fn get_images(post_id: String) -> Result<Vec<ProductImageData>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let images = clorinde::queries::admin_product::images()
        .bind(&db, &post_id)
        .all()
        .await
        .map_err(|e| lib::emsg(e, "Product images find"))?;
    let images: Vec<ProductImageData> = images
        .into_iter()
        .map(|data| ProductImageData {
            id: data.id.clone(),
            alt: data.alt.clone(),
            order: data.order,
            is_hero: data.is_hero,
        })
        .collect();
    Ok(images)
}

#[server(ImagesOrderUpdate, "/api")]
pub async fn images_order_update(
    ids: Vec<String>,
) -> Result<Result<(), AppError>, ServerFnError> {
    let mut db_trx = crate::server::db::use_db().await?;
    let order_update = ids.into_iter().enumerate();
    let trx = db_trx.transaction().await.map_err(|e| lib::emsg(e, "Images order transaction init"))?;
    for (i, id) in order_update {
        // TODO how this can be improved? batch, sql?
        clorinde::queries::product_image::update_order().bind(&trx, &(i as i32), &id).await?;
    }
    trx.commit().await.map_err(|e| lib::emsg(e, "Images order transaction"))?;
    Ok(Ok(()))
}

// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub struct ImageMakeHeroData {
//     pub hero: String,
//     pub not_hero: Option<String>,
// }
// pub type ImageMakeHeroResult = Result<ImageMakeHeroData, ImageLoadError>;
pub type ImageMakeHeroResult = Result<(), AppError>;
#[server(ImageMakeHero, "/api")]
pub async fn image_make_hero(id: String) -> Result<ImageMakeHeroResult, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let current_img = clorinde::queries::product_image::select_product_id()
        .bind(&db, &id)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Image find"))?;
    if let None = current_img {
        return Ok(Err(AppError::NotFound));
    }
    let current_img = current_img.unwrap();
    let current_hero = clorinde::queries::product_image::find_hero()
        .bind(&db, &current_img)
        .opt()
        .await
        .map_err(|e| lib::emsg(e, "Image current hero find"))?;

    let mut db_trx = crate::server::db::use_db().await?;
    let trx = db_trx.transaction().await.map_err(|e| lib::emsg(e, "Images hero transaction init"))?;
    clorinde::queries::product_image::set_hero()
        .bind(&db, &id)
        .await?;
    if let Some(current_hero) = current_hero {
        clorinde::queries::product_image::unset_hero()
            .bind(&db, &current_hero)
            .await?;
    }
    trx.commit().await.map_err(|e| lib::emsg(e, "Images hero transaction"))?;
    Ok(Ok(()))
}
