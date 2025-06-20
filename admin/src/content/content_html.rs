use leptos::prelude::*;

use crate::{content::{ContentEditor, ContentJsonUpdate}, form::FormFooter};

#[component]
pub fn ContentHtml(content_id: String, content_json: String) -> impl IntoView {
    let action = ServerAction::<ContentJsonUpdate>::new();
    let pending = action.pending();
    let (content, set_content) = signal(String::from(content_json.clone()));

    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Content</legend>
            <ContentEditor content_id=content_id.clone() content_json set_content />
            <ActionForm action=action>
                <input type="hidden" name="content_id" value=content_id />
                <input type="hidden" name="json" value=content />
                <FormFooter action=action submit_text="Save content" />
            </ActionForm>
        </fieldset>
    }
}

