use leptos::{either::Either, html::Dialog, prelude::*};
use reactive_stores::{Field, Patch, Store, StoreField, StoreFieldIterator};
use serde::{Deserialize, Serialize};

use crate::{
    form::FormFooter,
    image::{img_url_small, srcset_small, ImageLoadError},
    post::{
        ImageDelete, ImageEditData, ImageEditSignal, ImageUpdate, ImageUpload, PostImageModalForm,
    },
    util::{AlertDanger, Loading},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostImageData {
    pub id: String,
    pub alt: String,
    pub order: i32,
    pub is_hero: bool,
}

#[derive(Store, Debug, Clone)]
pub struct ImagesStore {
    #[store(key: String = |row| row.id.clone())]
    pub images: Vec<ImageStore>,
}
// impl Store<ImagesStore> {
//     pub fn set_hero(id: String) {}
// }

#[derive(Store, Debug, Clone)]
pub struct ImageStore {
    pub id: String,
    pub alt: String,
    pub order: i32,
    pub is_hero: bool,
}

#[component]
pub fn PostImages(post_id: String, images_store: Store<ImagesStore>) -> impl IntoView {
    let image_delete = ServerAction::<ImageDelete>::new();
    let image_upload = ServerAction::<ImageUpload>::new();
    let image_update = ServerAction::<ImageUpdate>::new();
    let order_action = ServerAction::<ImagesOrderUpdate>::new();
    let hero_action = ServerAction::<ImageMakeHero>::new();

    // let get_images_action = ServerAction::<GetImages>::new();
    // let get_images_action_value = get_images_action.value();

    // Effect::new(move |prev: Option<()>| {
    //     tracing::info!("images effect");
    //     if prev.is_some() {
    //         let v = get_images_action_value();
    //         match v {
    //             Some(Ok(v)) => {
    //                 images.set(v);
    //             }
    //             _ => todo!(),
    //         }
    //     }
    // });
    let images = images_store.images();
    Effect::new(move |_| {
        let v = hero_action.value().get();
        if let Some(v) = v {
            match v {
                Ok(Ok(v)) => {
                    // let images = images_store.images();
                    // let current_hero = images
                    //     .iter_unkeyed()
                    //     .find(|image| image.id().get() == v.hero);
                    let current_hero = images
                        .iter_unkeyed()
                        .find(|image| image.id().get_untracked() == v.hero);
                    if let Some(current_hero) = current_hero {
                        tracing::info!("current_hero={:?}", current_hero);
                        // current_hero.is_hero().update_untracke(|wtf| false);
                        // current_hero.is_hero().patch(false);
                    }
                }
                _ => todo!(),
            }
        }
    });

    // let post_id_clone2 = post_id.clone();
    // Effect::new(move |_| {
    //     tracing::info!("images effect");
    //     get_images_action.dispatch(GetImages {
    //         post_id: post_id_clone2.clone(),
    //     });
    //     (
    //         post_id_clone2.clone(),
    //         image_delete.version().get(),
    //         image_upload.version().get(),
    //         image_update.version().get(),
    //         order_action.version().get(),
    //         hero_action.version().get(),
    //     )
    // });

    view! {
        <ImageUpload post_id image_upload />
        <PostImagesView images_store image_delete image_update order_action hero_action />
    }
}

#[component]
pub fn PostImagesView(
    // images: Vec<PostImageData>,
    // images: RwSignal<Vec<PostImageData>>,
    images_store: Store<ImagesStore>,
    image_delete: ServerAction<ImageDelete>,
    image_update: ServerAction<ImageUpdate>,
    order_action: ServerAction<ImagesOrderUpdate>,
    hero_action: ServerAction<ImageMakeHero>,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let (editing, set_editing) = signal::<ImageEditSignal>(None);

    // let (images_sorted, set_images_sorted) = images.split();
    // let (images_sorted, set_images_sorted) = signal(images);

    let on_order = move |id: String, dir: i32| {
        // let il = images_sorted.get().clone();
        // let from_index = il.iter().position(|item| item.id == id).unwrap();
        // let to_index = from_index as i32 + dir;
        // set_images_sorted.update(|mut_il| {
        //     let removed_item = mut_il.remove(from_index);
        //     mut_il.insert(to_index as usize, removed_item);
        //     let images = mut_il
        //         .into_iter()
        //         .enumerate()
        //         .map(|(i, img)| PostImageData {
        //             order: i as i32,
        //             ..img.clone()
        //         })
        //         .collect::<Vec<_>>();
        //     *mut_il = images;
        // });
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

    let count = move || images_store.images().iter_unkeyed().count();
    let no_images = move || match count() {
        0 => Either::Left(view! { <p>No images were found.</p> }),
        _ => Either::Right(()),
    };

    view! {
        <fieldset prop:disabled=disabled>
            <legend>Images</legend>
            <ActionForm action=order_action>
                <For
                    each=move || images_store.images()
                    // key=|image| image.id().get()
                    key=|image| image.read().id.clone()
                    let:item
                >
                    // children=move |image| {
                    // let value = image.value();
                    // view! { <input type="hidden" name="ids[]" value=move || value.get() /> }
                    // }
                    <input type="hidden" name="ids[]" value=item.id().get() />
                </For>

                <FormFooter action=order_action submit_text="Save order" />
            </ActionForm>
            <div class="images">
                // <For
                // each=move || images_sorted()
                // key=|image| format!("{}:{}", image.id, image.order)
                // children=move |image: PostImageData| {
                {no_images}
                <For
                    each=move || images_store.images()
                    key=|image| image.read().id.clone()
                    children=move |image| {
                        let image_untracked = image.get_untracked();
                        let order = image_untracked.order;
                        let id = image_untracked.id;
                        let is_last = order + 1 == count() as i32;
                        let id_to_make_hero = id;
                        let make_hero = move || {
                            hero_action
                                .dispatch(ImageMakeHero {
                                    id: id_to_make_hero.clone(),
                                });
                        };
                        // let image_read = image.read();
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
    // image: PostImageData,
    #[prop(into)] image: Field<ImageStore>,
    set_editing: WriteSignal<ImageEditSignal>,
    on_order: F,
    is_last: bool,
    make_hero: H,
) -> impl IntoView
where
    F: Fn(String, i32) + 'static + Clone,
    H: Fn() + 'static + Clone,
{
    let image = image.read();
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
            <figcaption>{image.alt.clone()}</figcaption>
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
