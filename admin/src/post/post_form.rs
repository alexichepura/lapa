use chrono::{DateTime, FixedOffset};
use leptos::*;
use leptos_meta::Title;
use leptos_router::{use_navigate, ActionForm};
use serde::{Deserialize, Serialize};

use super::PostError;
use crate::{
    form::Input,
    post::PostImages,
    util::{datetime_to_html, datetime_to_strings, html_to_datetime, Pending, ResultAlert},
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostFormData {
    pub id: Option<String>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<FixedOffset>,
    pub published_at: DateTime<FixedOffset>,
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
    let (published_at, set_published_at) = create_slice(
        cx,
        post_rw,
        |state| state.published_at.clone(),
        |state, published_at| state.published_at = published_at,
    );

    // let (published_at, set_published_at) =
    //     create_signal::<DateTime<FixedOffset>>(cx, DateTime::from(DateTime::MIN_UTC));

    let html_published_at = create_memo(cx, move |_| datetime_to_html(published_at()));

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

    let created = datetime_to_strings(post.created_at);

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
                        <label>
                            <div>"Description"</div>
                            <textarea name="description" prop:value=post.description></textarea>
                        </label>
                    </div>
                    <div>
                        <div class="Grid-fluid-2">
                            <label>
                                <div>"Created at "<small>"(Local)"</small></div>
                                <input value=created.local disabled/>
                            </label>
                            <label>
                                <div>"Created at "<small>"(UTC)"</small></div>
                                <input value=created.utc disabled/>
                            </label>
                        </div>
                        <div class="Grid-fluid-2">
                            <label>
                                <div>"Published at "<small>"(Local)"</small></div>
                                <input
                                    value=html_published_at
                                    prop:value=html_published_at
                                    type="datetime-local"
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev);
                                        log!("{}", val);
                                        let dt = html_to_datetime(val);
                                        log!("{}", dt.to_string());
                                        set_published_at(dt);
                                    }
                                />
                                <input
                                    name="published_at_rfc3339"
                                    type="hidden"
                                    prop:value=move || published_at().to_rfc3339()
                                />
                                // <input
                                //     name="published_at"
                                //     type="hidden"
                                //     prop:value=html_published_at().to_string()
                                // />
                            </label>
                            <label>
                                <div>"Published at "<small>"(UTC)"</small></div>
                                <input value="" disabled/>
                            </label>
                        </div>
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
                                view! { cx, <ResultAlert result=post_result/> }.into_view(cx)
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
    published_at_rfc3339: DateTime<FixedOffset>,
) -> Result<Result<PostFormData, PostError>, ServerFnError> {
    dbg!(published_at_rfc3339);

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
                    db::post::published_at::set(published_at_rfc3339),
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
            created_at: post.created_at,
            published_at: post.published_at,
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
                    db::post::published_at::set(published_at_rfc3339),
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
            created_at: post.created_at,
            published_at: post.published_at,
        }));
    }
}
