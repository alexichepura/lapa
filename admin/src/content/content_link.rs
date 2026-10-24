use leptos::{either::Either, html::Dialog, prelude::*};
use serde::{Deserialize, Serialize};

use crate::form::Input;

stylance::import_crate_style!(
    #[allow(dead_code)]
    grid,
    "src/grid.module.css"
);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkEditData {
    pub href: String,
    pub text: String,
}

#[component]
pub fn ContentLinkEditor(
    editing: ReadSignal<Option<LinkEditData>>,
    on_save: Callback<LinkEditData>,
    on_delete: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let dialog_element: NodeRef<Dialog> = NodeRef::new();
    let edit_view = move || match editing() {
        Some(link) => Either::Left(view! {
            <LinkEditModalForm link on_save on_delete on_close />
        }),
        None => Either::Right(()),
    };
    Effect::new(move |_| {
        if editing().is_some() {
            let el = dialog_element.get().expect("<dialog> to exist");
            let _modal_result = el.show_modal();
        } else {
            let el = dialog_element.get();
            if let Some(el) = el {
                el.close();
            }
        }
    });
    view! {
        <dialog class=grid::gridFluid2 node_ref=dialog_element>
            {edit_view}
        </dialog>
    }
}

#[component]
fn LinkEditModalForm(
    link: LinkEditData,
    on_save: Callback<LinkEditData>,
    on_delete: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let href = RwSignal::new(link.href.clone());
    let text = RwSignal::new(link.text.clone());
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        on_save.run(LinkEditData {
            href: href.get_untracked(),
            text: text.get_untracked(),
        });
    };
    view! {
        <div>
            <button
                on:click=move |ev| {
                    ev.prevent_default();
                    on_delete.run(());
                }
            >
                Delete
            </button>
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
                on_close.run(());
            }>Cancel without saving</button>
        </div>
    }
}
