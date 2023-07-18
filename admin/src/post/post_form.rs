use chrono::{DateTime, FixedOffset, Utc};
use leptos::*;
use leptos_meta::Title;
use leptos_router::{use_navigate, ActionForm};
use serde::{Deserialize, Serialize};

use super::PostError;
use crate::{
    form::{Checkbox, FormFooter, Input},
    post::{PostDeleteForm, PostImages},
    settings::use_site_url,
    util::{
        datetime_to_local_html, datetime_to_string, datetime_to_strings, html_local_to_datetime,
    },
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostFormData {
    pub id: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub text: String,
}

#[component]
pub fn PostNew(cx: Scope) -> impl IntoView {
    let post = PostFormData::default();
    view! { cx, <PostForm post=post/> }
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
    let (slug, set_slug) = create_slice(
        cx,
        post_rw,
        |state| state.slug.clone(),
        |state, slug| state.slug = slug,
    );
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

    let id_view = match post.id.clone() {
        Some(id) => id,
        None => "".to_string(),
    };
    let gallery_view = match post.id.clone() {
        Some(id) => view! { cx, <PostImages post_id=id/> }.into_view(cx),
        None => view! { cx, <p>"Gallery is not available for not saved post"</p> }.into_view(cx),
    };
    let delete_view = match post.id.clone() {
        Some(id) => view! { cx, <PostDeleteForm id=id.clone() slug/> }.into_view(cx),
        None => ().into_view(cx),
    };

    let created = datetime_to_strings(post.created_at);
    let site_url = use_site_url(cx);
    let href = move || format!("{}/post/{}", &site_url(), &slug());

    let published_at_utc_string = create_memo(cx, move |_| match published_at() {
        Some(published_at) => datetime_to_string(published_at),
        None => String::default(),
    });
    view! { cx,
        <Title text=move || format!("Post: {}", title())/>
        <header>
            <div>
                <h1>"Post edit"</h1>
                <a href=move || href() target="_blank">
                    {move || href()}
                </a>
            </div>
            <dl>
                <dt>"ID: "</dt>
                <dd>{id_view}</dd>
                <br/>
                <dt>"Created at " <small>"(Local): "</small></dt>
                <dd>{created.local}</dd>
                <br/>
                <dt>"Created at " <small>"(UTC): "</small></dt>
                <dd>{created.utc}</dd>
                <br/>
                <dt>"Published at " <small>"(UTC): "</small></dt>
                <dd>{published_at_utc_string}</dd>
            </dl>
        </header>
        <ActionForm action=post_upsert>
            <fieldset disabled=move || pending()>
                <legend>"Data"</legend>
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
                        <label>
                            <div>"Slug"</div>
                            <input
                                name="slug"
                                value=slug
                                on:input=move |ev| {
                                    set_slug(event_target_value(&ev));
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
                        <PublishedAt published_at set_published_at/>
                        <label>
                            <div>"Text"</div>
                            <textarea
                                name="text"
                                value=&post.text
                                prop:value=post.text
                                rows="6"
                            ></textarea>
                        </label>
                    </div>
                </div>
                <FormFooter action=post_upsert/>
            </fieldset>
        </ActionForm>
        {gallery_view}
        <div class="Grid-fluid-2">
            {delete_view}
        </div>
    }
}

#[component]
pub fn PublishedAt(
    cx: Scope,
    published_at: Signal<Option<DateTime<FixedOffset>>>,
    set_published_at: SignalSetter<Option<DateTime<FixedOffset>>>,
) -> impl IntoView {
    let is_published = create_memo(cx, move |_| published_at.with(|p| p.is_some()));
    let disabled = create_memo(cx, move |_| !is_published());

    let (is_published_signal, set_is_published) =
        create_signal(cx, published_at.get_untracked().is_some());

    create_effect(cx, move |old| {
        let is = is_published_signal();
        if old.is_some() {
            if is {
                set_published_at(Some(Utc::now().fixed_offset()));
            } else {
                set_published_at(None);
            }
        }
        is
    });

    let html_published_at = create_memo(cx, move |_| match published_at() {
        Some(published_at) => datetime_to_local_html(published_at),
        None => String::default(),
    });
    let published_at_rfc3339 = create_memo(cx, move |_| match published_at() {
        Some(published_at) => published_at.to_rfc3339(),
        None => String::default(),
    });

    view! { cx,
        <div class="Grid-fluid-2">
            <Checkbox label="Publish" checked=is_published set=set_is_published/>
            <label>
                <div>"Published at " <small>"(Local)"</small></div>
                <input
                    disabled=disabled
                    prop:disabled=disabled
                    value=html_published_at
                    prop:value=html_published_at
                    type="datetime-local"
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        let datetime = html_local_to_datetime(&val);
                        set_published_at(Some(datetime));
                    }
                />
                <input
                    name="published_at"
                    type="hidden"
                    value=published_at_rfc3339
                    prop:value=published_at_rfc3339
                />
            </label>
        </div>
    }
}

#[server(PostUpsert, "/api")]
pub async fn post_upsert(
    cx: Scope,
    id: Option<String>,
    published_at: Option<DateTime<FixedOffset>>,
    title: String,
    slug: String,
    description: String,
    text: String,
) -> Result<Result<PostFormData, PostError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let post_by_slug = prisma_client
        .post()
        .find_unique(db::post::slug::equals(slug.clone()))
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
                db::post::id::equals(id),
                vec![
                    db::post::published_at::set(published_at),
                    db::post::slug::set(slug),
                    db::post::title::set(title),
                    db::post::description::set(description),
                    db::post::text::set(text),
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
            created_at: post.created_at,
            published_at: post.published_at,
            slug: post.slug,
            title: post.title,
            description: post.description,
            text: post.text,
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
                    db::post::published_at::set(published_at),
                    db::post::title::set(title),
                    db::post::description::set(description),
                    db::post::text::set(text),
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
            text: post.text,
        }));
    }
}
