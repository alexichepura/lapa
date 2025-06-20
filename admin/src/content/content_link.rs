use leptos::{either::Either, html::Dialog, prelude::*};
use serde::{Deserialize, Serialize};

use crate::form::Input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkEditData {
    pub href: String,
    pub text: String,
}
#[component]
pub fn ContentLinkEditor(
    editing: ReadSignal<Option<LinkEditData>>,
    set_editing: WriteSignal<Option<LinkEditData>>,
    updater: ReadSignal<Option<Box<dyn Fn(LinkEditData)>>, LocalStorage>,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let save = move |updated_link: LinkEditData| {
        let binding = updater.read_untracked();
        let updater = binding.as_ref().unwrap();
        updater(updated_link);
        set_editing(None);
    };
    let del = move || {
        set_editing(None);
    };
    let close = move || {
        set_editing(None);
    };
    let edit_view = move || match editing() {
        Some(link) => Either::Left(view! { <LinkEditModalForm link save del close /> }),
        None => Either::Right(()),
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
    view! {
        <dialog class="Grid-fluid-2" node_ref=dialog_element>
            {edit_view}
        </dialog>
    }
}
#[component]
pub fn LinkEditModalForm(
    link: LinkEditData,
    save: impl Fn(LinkEditData) + 'static,
    del: impl Fn() + 'static,
    close: impl Fn() + 'static,
) -> impl IntoView {
    let href = RwSignal::new(link.href.clone());
    let text = RwSignal::new(link.text.clone());
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let updated_link = LinkEditData {
            href: href.get_untracked(),
            text: text.get_untracked(),
        };
        save(updated_link);
    };
    view! {
        <div>
            <button on:click=move |_e| del()>Delete</button>
            <hr />
            <form on:submit=on_submit>
                <fieldset>
                    <Input name="href" label="Href" value=link.href bind=href />
                    <Input name="text" label="Text" value=link.text bind=text />
                </fieldset>
                <button type="submit">Ok</button>
            </form>
            <button on:click=move |ev| {
                ev.prevent_default();
                close();
            }>Cancel without saving</button>
        </div>
    }
}
