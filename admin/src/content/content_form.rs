use chrono::{DateTime, FixedOffset, Utc};
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use serde::{Deserialize, Serialize};

use crate::{
    form::{Checkbox, FormFooter},
    util::{datetime_to_local_html, html_local_to_datetime},
};

use super::ContentError;

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentFormData {
    pub content_id: String,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub title: String,
    pub description: String,
}

#[component]
pub fn ContentForm(
    content: ContentFormData,
    title: Signal<String>,
    set_title: SignalSetter<String>,
    published_at: Signal<Option<DateTime<FixedOffset>>>,
    set_published_at: SignalSetter<Option<DateTime<FixedOffset>>>,
) -> impl IntoView {
    let content_update = ServerAction::<ContentUpdate>::new();
    let pending = content_update.pending();
    // let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    // let memo_fn = move |_| match published_at() {
    //     Some(published_at) => Either::Left(datetime_to_string(published_at).into_any()),
    //     None => Either::Right(().into_any()),
    // };
    // let published_at_utc_string = create_memo(memo_fn);
    // let published_at_utc_string = Memo::new(move |_| match published_at() {
    //     Some(published_at) => datetime_to_string(published_at).into_any(),
    //     None => ().into_any(),
    // });
    // let published_at_utc_string = create_memo(move |_| match published_at() {
    //     Some(published_at) => Either::Left(datetime_to_string(published_at).into_any()),
    //     None => Either::Right(().into_any()),
    // });
    view! {
        <ActionForm action=content_update>
            <input type="hidden" name="content_id" value=content.content_id />
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
                            <div>Description</div>
                            <textarea name="description" prop:value=content.description></textarea>
                        </label>
                    </div>
                    <div>
                        <PublishedAt published_at set_published_at />
                    </div>
                </div>
                <FormFooter action=content_update submit_text="Submit post data" />
            </fieldset>
        </ActionForm>
    }
}

#[component]
pub fn PublishedAt(
    published_at: Signal<Option<DateTime<FixedOffset>>>,
    set_published_at: SignalSetter<Option<DateTime<FixedOffset>>>,
) -> impl IntoView {
    let is_published = Memo::new(move |_| published_at.with(|p| p.is_some()));
    let disabled = Memo::new(move |_| !is_published());

    // let (is_published_signal, set_is_published) = signal(published_at.get_untracked().is_some());
    let published_at_rw_signal = RwSignal::new(published_at.get_untracked().is_some());

    Effect::new(move |old: Option<bool>| {
        // let is = is_published_signal();
        let is = published_at_rw_signal.get();
        tracing::info!("published_at_rw_signal={}", is);
        if old.is_some() {
            if is {
                set_published_at(Some(Utc::now().fixed_offset()));
            } else {
                set_published_at(None);
            }
        }
        is
    });

    let html_published_at = Memo::new(move |_| match published_at() {
        Some(published_at) => datetime_to_local_html(published_at),
        None => String::default(),
    });
    let published_at_rfc3339 = Memo::new(move |_| match published_at() {
        Some(published_at) => published_at.to_rfc3339(),
        None => String::default(),
    });

    view! {
        <div class="Grid-fluid-2">
            <Checkbox
                label="Publish"
                bind=published_at_rw_signal
                checked=published_at_rw_signal.get_untracked()
            />
            <label>
                <div>Published at <small>(Local)</small></div>
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

#[server(ContentUpdate, "/api")]
pub async fn content_update(
    content_id: String,
    published_at: Option<DateTime<FixedOffset>>,
    title: String,
    description: String,
) -> Result<Result<(), ContentError>, ServerFnError> {
    use prisma_web_client::db;
    let prisma_web_client = crate::server::use_prisma_web()?;

    let _ = prisma_web_client
        .content()
        .update(
            db::content::id::equals(content_id),
            vec![
                db::content::published_at::set(published_at),
                db::content::title::set(title),
                db::content::description::set(description),
            ],
        )
        .select(db::content::select!({ id }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Content update"))?;

    return Ok(Ok(()));
}
