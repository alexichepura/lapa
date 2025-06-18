use chrono::{DateTime, FixedOffset, Utc};
use leptos::{either::Either, prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

use super::PostError;
use crate::{
    form::{Checkbox, FormFooter}, settings::use_site_url, util::{
        datetime_to_local_html, datetime_to_string, datetime_to_strings, html_local_to_datetime,
    }
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostFormData {
    pub id: String,
    pub created_at: DateTime<FixedOffset>,
    pub publish_at: Option<DateTime<FixedOffset>>,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub h1: String,
}

#[component]
pub fn PostForm(post: PostFormData) -> impl IntoView {
    let action = ServerAction::<PostUpdate>::new();
    let pending = action.pending();

    let post_rw = RwSignal::new(post.clone());
    let (slug, set_slug) = create_slice(
        post_rw,
        |state| state.slug.clone(),
        |state, slug| state.slug = slug,
    );
    let (title, set_title) = create_slice(
        post_rw,
        |state| state.meta_title.clone(),
        |state, title| state.meta_title = title,
    );
    let (publish_at, set_publish_at) = create_slice(
        post_rw,
        |state| state.publish_at.clone(),
        |state, publish_at| state.publish_at = publish_at,
    );

    let created = datetime_to_strings(post.created_at);
    let site_url = move || use_site_url();
    let href = move || format!("{}/post/{}", &site_url(), &slug());
    let publish_at_utc_string = move || match publish_at() {
        Some(publish_at) => Either::Left(datetime_to_string(publish_at)),
        None => Either::Right(()),
    };

    let id = post.id.clone();
    let id_value = post.id.clone();
    view! {
        <Title text=move || format!("Post: {}", title()) />
        <section class="PostPage">
            <header>
                <div>
                    <h1>Post</h1>
                    <a href=move || href() target="_blank">
                        {move || href()}
                    </a>
                </div>
                <dl>
                    <dt>ID:</dt>
                    <dd>{id.clone()}</dd>
                    <br />
                    <dt>Created at <small>(UTC):</small></dt>
                    <dd>{created.utc}</dd>
                    <br />
                    <dt>Published at <small>(UTC):</small></dt>
                    <dd>{publish_at_utc_string}</dd>
                </dl>
            </header>
            <ActionForm action=action>
                <input type="hidden" name="id" value=id_value />
                <fieldset prop:disabled=move || pending()>
                    <legend>Data</legend>
                    <div class="Grid-fluid-2">
                        <div>
                            <label>
                                <div>Title</div>
                                <input
                                    name="title"
                                    value=title
                                    on:input=move |ev| {
                                        set_title(event_target_value(&ev));
                                    }
                                />

                            </label>
                            <label>
                                <div>Slug</div>
                                <input
                                    name="slug"
                                    value=slug
                                    on:input=move |ev| {
                                        set_slug(event_target_value(&ev));
                                    }
                                />

                            </label>
                            <label>
                                <div>Description</div>
                                <textarea
                                    name="description"
                                    prop:value=post.meta_description
                                ></textarea>
                            </label>
                        </div>
                        <div>
                            <PublishAt publish_at set_publish_at />
                        </div>
                    </div>
                    <FormFooter action=action submit_text="Submit post data" />
                </fieldset>
            </ActionForm>
        </section>
    }
}

#[component]
pub fn PublishAt(
    publish_at: Signal<Option<DateTime<FixedOffset>>>,
    set_publish_at: SignalSetter<Option<DateTime<FixedOffset>>>,
) -> impl IntoView {
    let is_published = Memo::new(move |_| publish_at.with(|p| p.is_some()));
    let disabled = Memo::new(move |_| !is_published());
    let publish_at_rw_signal = RwSignal::new(publish_at.get_untracked().is_some());

    Effect::new(move |old: Option<bool>| {
        let is = publish_at_rw_signal.get();
        tracing::info!("published_at_rw_signal={}", is);
        if old.is_some() {
            if is {
                set_publish_at(Some(Utc::now().fixed_offset()));
            } else {
                set_publish_at(None);
            }
        }
        is
    });

    let html_publish_at = Memo::new(move |_| match publish_at() {
        Some(publish_at) => datetime_to_local_html(publish_at),
        None => String::default(),
    });
    let publish_at_rfc3339 = Memo::new(move |_| match publish_at() {
        Some(publish_at) => publish_at.to_rfc3339(),
        None => String::default(),
    });

    view! {
        <div class="Grid-fluid-2">
            <Checkbox
                label="Publish"
                bind=publish_at_rw_signal
                checked=publish_at_rw_signal.get_untracked()
            />
            <label>
                <div>Published at <small>(Local)</small></div>
                <input
                    disabled=disabled
                    prop:disabled=disabled
                    value=html_publish_at
                    prop:value=html_publish_at
                    type="datetime-local"
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        let datetime = html_local_to_datetime(&val);
                        set_publish_at(Some(datetime));
                    }
                />

                <input
                    name="publish_at"
                    type="hidden"
                    value=publish_at_rfc3339
                    prop:value=publish_at_rfc3339
                />
            </label>
        </div>
    }
}

#[server(PostUpdate, "/api")]
pub async fn post_update(
    id: String,
    publish_at: Option<DateTime<FixedOffset>>,
    title: String,
    slug: String,
    description: String,
    h1: String,
) -> Result<Result<(), PostError>, ServerFnError> {
    use clorinde::queries;
    let db = crate::server::db::use_db().await?;
    let post_by_slug_id = clorinde::queries::admin_post::by_slug()
        .bind(&db, &slug).opt()
        .await
        .map_err(|e| lib::emsg(e, "Post by slug"))?;

    if let Some(post_by_slug_id) = post_by_slug_id {
        if id != post_by_slug_id {
            tracing::warn!("Post exists for slug={}", slug);
            return Ok(Err(PostError::CreateSlugExists));
        }
    }
    queries::admin_post::update()
        .bind(&db, &publish_at.map(|publish_at| publish_at.naive_utc()), &slug, &title, &description, &h1, &id)
        .await
        .map_err(|e| lib::emsg(e, "Post update"))?;
    return Ok(Ok(()));
}
