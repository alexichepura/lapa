use leptos::*;
use leptos_meta::Title;
use leptos_router::{use_navigate, ActionForm};
use serde::{Deserialize, Serialize};

use super::PostError;
use crate::{
    form::Input,
    post::PostImages,
    util::{Pending, ResultAlert},
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostFormData {
    pub id: Option<String>,
    pub slug: String,
    pub title: String,
    pub description: String,
}

// impl Default for PostFormData {
//     fn default() -> Self {
//         Self {
//             id: None,
//             slug: None,
//             title: None,
//             description: None,
//         }
//     }
// }

#[component]
pub fn PostNew(cx: Scope) -> impl IntoView {
    let post = PostFormData::default();
    view! { cx,
        <h1>"Post create"</h1>
        <PostForm post=post/>
    }
}

#[component]
pub fn PostForm(cx: Scope, post: PostFormData) -> impl IntoView {
    let post_upsert = create_server_action::<PostUpsert>(cx);
    let value = post_upsert.value();
    let pending = post_upsert.pending();
    // let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    if let None = post.id {
        create_effect(cx, move |_| {
            log!("navigate create_effect run");
            let v = value();
            if let Some(v) = v {
                let post_result = v.map_err(|_| PostError::ServerError).flatten();
                if let Ok(post_result) = post_result {
                    log!("navigate post_result ok");
                    let navigate = use_navigate(cx);
                    let to = format!("/posts/{}", post_result.id.unwrap());
                    request_animation_frame(move || {
                        // see use_navigate docs
                        // RAF prevents action signal update warning
                        log!("navigate request_animation_frame");
                        navigate(&to, Default::default()).expect("post route");
                    });
                }
            }
        });
    }

    let post_rw = create_rw_signal(cx, post.clone());
    let (title, set_title) = create_slice(
        cx,
        post_rw,
        |state| state.title.clone(),
        |state, title| state.title = title,
    );

    // let header_view = if let Some(id) = &post.id {
    //     view! { cx, <h2>"ID: " <small>{id}</small></h2> }.into_view(cx)
    // } else {
    //     ().into_view(cx)
    // };
    // let header_view = if let Some(id) = &post.id {
    //     view! { cx, "ID: "{id} }.into_view(cx)
    // } else {
    //     ().into_view(cx)
    // };
    let header_view = if let Some(id) = &post.id {
        format!("update {}", id.clone())
    } else {
        "create new".to_string()
    };
    let id_view = if let Some(id) = &post.id {
        view! { cx, <input type="hidden" name="id" value=id/> }.into_view(cx)
    } else {
        ().into_view(cx)
    };
    let gallery_view = if let Some(id) = post.id {
        view! { cx, <PostImages post_id=id/> }.into_view(cx)
    } else {
        view! { cx, <p>"Gallery is not available for not saved post"</p> }.into_view(cx)
    };

    view! { cx,
        <Title text=move || format!("Post: {}", title())/>
        <ActionForm action=post_upsert>
            <fieldset disabled=move || pending()>
                <legend>{header_view}</legend>
                {id_view}
                <div class="Grid-fluid-2">
                    <div>
                        <label>
                            <div>"Title"</div>
                            <input
                                name="title"
                                value=title
                                on:input=move |ev| {
                                    set_title(event_target_value(&ev));
                                }
                            />
                        </label>
                        <Input name="slug" label="Slug" value=post.slug/>
                    </div>
                    <div>
                        <label>
                            <div>"Description"</div>
                            <textarea name="description" prop:value=post.description></textarea>
                        </label>
                    </div>
                </div>
                <footer>
                    <input type="submit" value="SUBMIT"/>
                    <Pending pending/>
                    <Suspense fallback=|| ()>
                        {move || match value() {
                            None => ().into_view(cx),
                            Some(v) => {
                                let post_result = v.map_err(|_| PostError::ServerError).flatten();
                                view! { cx, <ResultAlert result=post_result/>}.into_view(cx)
                            }
                        }}
                    </Suspense>
                </footer>
            </fieldset>
        </ActionForm>
        {gallery_view}
    }
}

#[server(PostUpsert, "/api")]
pub async fn post_upsert(
    cx: Scope,
    id: Option<String>,
    title: String,
    slug: String,
    description: String,
) -> Result<Result<PostFormData, PostError>, ServerFnError> {
    dbg!((title.clone(), slug.clone(), description.clone()));
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let post_by_slug = prisma_client
        .post()
        .find_unique(db::post::UniqueWhereParam::SlugEquals(slug.clone()))
        .select(db::post::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    if let Some(id) = id {
        if let Some(_post_by_slug) = post_by_slug {
            if id != _post_by_slug.id {
                dbg!(_post_by_slug);
                return Ok(Err(PostError::CreateSlugExists));
            }
        }
        let post = prisma_client
            .post()
            .update(
                db::post::UniqueWhereParam::IdEquals(id),
                vec![
                    db::post::slug::set(slug),
                    db::post::title::set(title),
                    db::post::description::set(description),
                ],
            )
            .exec()
            .await
            .map_err(|e| {
                dbg!(e);
                ServerFnError::ServerError("Server error".to_string())
            })?;
        return Ok(Ok(PostFormData {
            id: Some(post.id),
            slug: post.slug,
            title: post.title,
            description: post.description,
        }));
    } else {
        if let Some(_post_by_slug) = post_by_slug {
            dbg!(_post_by_slug);
            return Ok(Err(PostError::CreateSlugExists));
        }
        let post = prisma_client
            .post()
            .create(
                slug,
                vec![
                    db::post::title::set(title),
                    db::post::description::set(description),
                ],
            )
            .exec()
            .await
            .map_err(|e| {
                dbg!(e);
                ServerFnError::ServerError("Server error".to_string())
            })?;
        return Ok(Ok(PostFormData {
            id: Some(post.id),
            slug: post.slug,
            title: post.title,
            description: post.description,
        }));
    }
}
