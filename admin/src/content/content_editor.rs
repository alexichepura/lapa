use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptoslate::core::{FeatureSet, PropertyPath};
use leptoslate::{Editor, EditorHandle, Element, MarkSet, Node, Props, Range, Renderers};
use leptoslate_rich::{ListHandle, RichConfig, RichEditor, RichListFeature, RichTableFeature};
use serde_json::{Value, json};

use crate::content::{ContentImageEditor, ContentLinkEditor, ImageEditData, LinkEditData};

/// Marks used by the existing documents (`strong` / `em` / `u`).
fn marks() -> MarkSet {
    MarkSet::new().bold("strong").italic("em").underline("u")
}

/// Links are stored as `{ "type": "a", "attributes": { "href": … } }`.
fn link_property() -> PropertyPath {
    PropertyPath::new("attributes.href")
}

#[component]
pub fn ContentEditor(
    content_id: String,
    content_json: String,
    set_content: WriteSignal<String>,
) -> impl IntoView {
    let marks = marks();
    let link_property = link_property();
    let features = FeatureSet::new()
        .with(RichListFeature)
        .with(RichTableFeature);

    let nodes = serde_json::from_str::<Vec<Node>>(&content_json)
        .unwrap_or_else(|_| vec![Node::Element(Element::paragraph(vec![Node::empty_text()]))]);
    let editor = EditorHandle::from_editor(
        Editor::new_with_features(nodes, features).with_link_property(link_property.clone()),
    );

    let (link_editing, set_link_editing) = signal::<Option<LinkEditData>>(None);
    let link_selection = RwSignal::new(None::<Range>);
    let link_existing = RwSignal::new(false);

    let (image_editing, set_image_editing) = signal::<Option<ImageEditData>>(None);
    let image_selection = RwSignal::new(None::<Range>);
    let image_path = RwSignal::new(None::<Vec<usize>>);

    let renderers = image_renderers(set_image_editing, image_path);

    let link_editor = editor.clone();
    let open_link = Callback::new(move |_| {
        let selection = link_editor
            .state
            .with_untracked(|state| state.selection.clone());
        let selected_text = link_editor.selection_string().unwrap_or_default();
        let (href, text, existing) = match link_editor.link_node_at_selection() {
            Some((path, href)) => {
                let text = link_editor
                    .read(|state| state.node(&path).map(Node::string).unwrap_or_default());
                (href, text, true)
            }
            None => (String::new(), selected_text, false),
        };
        link_selection.set(selection);
        link_existing.set(existing);
        set_link_editing.set(Some(LinkEditData { href, text }));
    });

    let save_link_editor = editor.clone();
    let save_link_property = link_property.clone();
    let on_link_save = Callback::new(move |data: LinkEditData| {
        if let Some(selection) = link_selection.get_untracked() {
            save_link_editor.update(|state| state.selection = Some(selection));
        }
        let href = data.href.trim().to_string();
        if !href.is_empty() {
            if link_existing.get_untracked() {
                let _ = save_link_editor.update_link_at_selection(&href, &data.text);
            } else {
                let collapsed = save_link_editor.state.with_untracked(|state| {
                    state
                        .selection
                        .as_ref()
                        .map(|range| range.is_collapsed())
                        .unwrap_or(true)
                });
                if collapsed {
                    let mut properties = Props::new();
                    save_link_property.set(&mut properties, json!(href));
                    let node = Node::Element(
                        Element::new("a", vec![Node::text(data.text.clone())])
                            .with_properties(properties),
                    );
                    save_link_editor.insert_inline_node(node);
                } else {
                    let _ = save_link_editor.set_link(&href);
                    if !data.text.is_empty() {
                        let _ = save_link_editor.update_link_at_selection(&href, &data.text);
                    }
                }
            }
        }
        set_link_editing.set(None);
    });

    let delete_link_editor = editor.clone();
    let on_link_delete = Callback::new(move |_| {
        if let Some(selection) = link_selection.get_untracked() {
            delete_link_editor.update(|state| state.selection = Some(selection));
        }
        delete_link_editor.remove_link();
        set_link_editing.set(None);
    });
    let on_link_close = Callback::new(move |_| set_link_editing.set(None));

    let open_image_editor = editor.clone();
    let open_image = Callback::new(move |_| {
        image_selection.set(
            open_image_editor
                .state
                .with_untracked(|state| state.selection.clone()),
        );
        image_path.set(None);
        set_image_editing.set(Some(ImageEditData::default()));
    });

    let save_image_editor = editor.clone();
    let on_image_save = Callback::new(move |data: ImageEditData| {
        let path = image_path.get_untracked();
        // A new image needs an upload; editing an existing (possibly external)
        // image may only change alt/caption.
        if path.is_none() && data.id.trim().is_empty() {
            set_image_editing.set(None);
            return;
        }
        if let Some(path) = path {
            let mut properties = Props::new();
            properties.insert("id".into(), json!(data.id));
            properties.insert("alt".into(), json!(data.alt));
            properties.insert("caption".into(), json!(data.caption));
            save_image_editor.update(|state| state.set_node_properties(&path, properties));
        } else {
            if let Some(selection) = image_selection.get_untracked() {
                save_image_editor.update(|state| state.selection = Some(selection));
            }
            let node = Node::Element(
                Element::new("img", vec![Node::empty_text()])
                    .with_property("id", json!(data.id))
                    .with_property("alt", json!(data.alt))
                    .with_property("caption", json!(data.caption)),
            );
            save_image_editor.update(|state| {
                state.insert_block_after_selection(node);
            });
        }
        set_image_editing.set(None);
    });

    let delete_image_editor = editor.clone();
    let on_image_delete = Callback::new(move |_| {
        if let Some(path) = image_path.get_untracked() {
            delete_image_editor.update(|state| {
                let _ = state.remove_block_at(&path);
            });
        }
        set_image_editing.set(None);
    });
    let on_image_close = Callback::new(move |_| set_image_editing.set(None));

    let config = RichConfig::new()
        .with_marks(marks.clone())
        .with_link_property(link_property)
        .with_on_change(Callback::new(move |json: String| set_content.set(json)))
        .with_renderers(renderers)
        .with_hover_bar(true)
        .with_selection_bubble(true);

    view! {
        <div class="content-editor">
            <ContentToolbar
                editor=editor.clone()
                marks=marks
                on_link=open_link
                on_image=open_image
            />
            <RichEditor editor=editor.clone() config=config placeholder="Placeholder..." />
        </div>
        <ContentLinkEditor
            editing=link_editing
            on_save=on_link_save
            on_delete=on_link_delete
            on_close=on_link_close
        />
        <ContentImageEditor
            content_id=content_id
            editing=image_editing
            on_save=on_image_save
            on_delete=on_image_delete
            on_close=on_image_close
        />
    }
}

fn image_renderers(
    set_image_editing: WriteSignal<Option<ImageEditData>>,
    image_path: RwSignal<Option<Vec<usize>>>,
) -> Renderers {
    Renderers::new().element("img", move |ctx| {
        let id = ctx.prop_str("id").unwrap_or("").to_string();
        let src = if id.is_empty() {
            ctx.prop_str("src").unwrap_or("").to_string()
        } else {
            format!("/content-image/{id}")
        };
        let alt = ctx.prop_str("alt").unwrap_or("").to_string();
        let caption = ctx.prop_str("caption").unwrap_or("").to_string();
        let path = ctx.path.clone();
        let element_type = ctx.element_type().to_string();
        let base_class = ctx.class_with("ls-media");
        let node_path = ctx.node_path;
        let data = ImageEditData {
            id,
            alt: alt.clone(),
            caption: caption.clone(),
        };

        let select_editor = ctx.editor.clone();
        let select_path = path.clone();
        let class_editor = ctx.editor.clone();
        let class_path = path.clone();
        let class = move || {
            if class_editor.is_node_selected(&class_path) {
                format!("{base_class} ls-selected")
            } else {
                base_class.clone()
            }
        };

        let edit_path = path.clone();
        let delete_editor = ctx.editor.clone();
        let delete_path = path;

        view! {
            <figure
                class=class
                data-node-path=node_path
                data-type=element_type
                contenteditable="false"
                on:click=move |ev: MouseEvent| {
                    ev.prevent_default();
                    select_editor.select_node(&select_path);
                }
            >
                <img src=src alt=alt draggable="false" />
                <figcaption>
                    {(!caption.is_empty()).then(|| view! { <span>{caption.clone()}</span> })}
                    <span class="ls-media-tools">
                        <button
                            type="button"
                            contenteditable="false"
                            title="Edit image"
                            aria-label="Edit image"
                            on:mousedown=move |ev: MouseEvent| ev.prevent_default()
                            on:click=move |ev: MouseEvent| {
                                ev.prevent_default();
                                ev.stop_propagation();
                                image_path.set(Some(edit_path.clone()));
                                set_image_editing.set(Some(data.clone()));
                            }
                        >
                            "Edit"
                        </button>
                        <button
                            type="button"
                            contenteditable="false"
                            title="Delete image"
                            aria-label="Delete image"
                            on:mousedown=move |ev: MouseEvent| ev.prevent_default()
                            on:click=move |ev: MouseEvent| {
                                ev.prevent_default();
                                ev.stop_propagation();
                                delete_editor.update(|state| {
                                    let _ = state.remove_block_at(&delete_path);
                                });
                            }
                        >
                            "✕"
                        </button>
                    </span>
                </figcaption>
            </figure>
        }
        .into_any()
    })
}

#[component]
fn ContentToolbar(
    editor: EditorHandle,
    marks: MarkSet,
    on_link: Callback<()>,
    on_image: Callback<()>,
) -> impl IntoView {
    let bold = marks.bold_key().to_string();
    let italic = marks.italic_key().to_string();
    let underline = marks.underline_key().to_string();
    let code = marks.code_key().to_string();

    view! {
        <div class="content-toolbar" role="toolbar" aria-label="Content formatting">
            <MarkButton editor=editor.clone() mark=bold label="B" title="Bold" />
            <MarkButton editor=editor.clone() mark=italic label="I" title="Italic" />
            <MarkButton editor=editor.clone() mark=underline label="U" title="Underline" />
            <MarkButton editor=editor.clone() mark=code label="C" title="Inline code" />
            <BlockButton editor=editor.clone() block_type="h1" label="H1" />
            <BlockButton editor=editor.clone() block_type="h2" label="H2" />
            <BlockButton editor=editor.clone() block_type="h3" label="H3" />
            <BlockButton editor=editor.clone() block_type="h4" label="H4" />
            <BlockButton editor=editor.clone() block_type="blockquote" label="❝" title="Quote" />
            <ListButton editor=editor.clone() list_type="ul" label="•" title="Bulleted list" />
            <ListButton editor=editor.clone() list_type="ol" label="1." title="Numbered list" />
            <button
                type="button"
                title="Link"
                aria-label="Link"
                on:mousedown=move |ev: MouseEvent| ev.prevent_default()
                on:click=move |_| on_link.run(())
            >
                "🔗"
            </button>
            <button
                type="button"
                title="Image"
                aria-label="Image"
                on:mousedown=move |ev: MouseEvent| ev.prevent_default()
                on:click=move |_| on_image.run(())
            >
                "🖼"
            </button>
            <UndoButton editor=editor.clone() />
            <RedoButton editor=editor />
        </div>
    }
}

#[component]
fn MarkButton(
    editor: EditorHandle,
    mark: String,
    label: &'static str,
    #[prop(default = "")] title: &'static str,
) -> impl IntoView {
    let click_editor = editor.clone();
    let click_mark = mark.clone();
    let active_editor = editor;
    let active = Memo::new(move |_| {
        active_editor
            .state
            .with(|state| state.has_mark(&mark, &Value::Bool(true)))
    });
    let title = if title.is_empty() { label } else { title };

    view! {
        <button
            type="button"
            class:active=move || active.get()
            aria-pressed=move || if active.get() { "true" } else { "false" }
            aria-label=title
            title=title
            on:mousedown=move |ev: MouseEvent| ev.prevent_default()
            on:click=move |_| click_editor.toggle_mark(&click_mark, Value::Bool(true))
        >
            {label}
        </button>
    }
}

#[component]
fn BlockButton(
    editor: EditorHandle,
    block_type: &'static str,
    label: &'static str,
    #[prop(default = "")] title: &'static str,
) -> impl IntoView {
    let click_editor = editor.clone();
    let active_editor = editor;
    let active = Memo::new(move |_| {
        active_editor.state.with(|state| {
            state
                .selection
                .as_ref()
                .and_then(|range| state.leaf_block_path(range.start()))
                .and_then(|path| state.element(&path))
                .map(|element| element.element_type == block_type)
                .unwrap_or(false)
        })
    });
    let title = if title.is_empty() { label } else { title };

    view! {
        <button
            type="button"
            class:active=move || active.get()
            aria-pressed=move || if active.get() { "true" } else { "false" }
            aria-label=title
            title=title
            on:mousedown=move |ev: MouseEvent| ev.prevent_default()
            on:click=move |_| click_editor.set_block(block_type)
        >
            {label}
        </button>
    }
}

#[component]
fn ListButton(
    editor: EditorHandle,
    list_type: &'static str,
    label: &'static str,
    #[prop(default = "")] title: &'static str,
) -> impl IntoView {
    let click_editor = editor.clone();
    let active_editor = editor;
    let active = Memo::new(move |_| {
        active_editor.state.with(|state| {
            let Some(range) = &state.selection else {
                return false;
            };
            let top = range.start().path.first().copied().unwrap_or(0);
            state
                .element(&[top])
                .map(|element| element.element_type == list_type)
                .unwrap_or(false)
        })
    });
    let title = if title.is_empty() { label } else { title };

    view! {
        <button
            type="button"
            class:active=move || active.get()
            aria-pressed=move || if active.get() { "true" } else { "false" }
            aria-label=title
            title=title
            on:mousedown=move |ev: MouseEvent| ev.prevent_default()
            on:click=move |_| click_editor.toggle_list(list_type)
        >
            {label}
        </button>
    }
}

#[component]
fn UndoButton(editor: EditorHandle) -> impl IntoView {
    let click_editor = editor.clone();
    let active_editor = editor;
    let can_undo = Memo::new(move |_| active_editor.state.with(|state| state.can_undo()));

    view! {
        <button
            type="button"
            title="Undo"
            aria-label="Undo"
            prop:disabled=move || !can_undo.get()
            on:mousedown=move |ev: MouseEvent| ev.prevent_default()
            on:click=move |_| {
                click_editor.undo();
            }
        >
            "↶"
        </button>
    }
}

#[component]
fn RedoButton(editor: EditorHandle) -> impl IntoView {
    let click_editor = editor.clone();
    let active_editor = editor;
    let can_redo = Memo::new(move |_| active_editor.state.with(|state| state.can_redo()));

    view! {
        <button
            type="button"
            title="Redo"
            aria-label="Redo"
            prop:disabled=move || !can_redo.get()
            on:mousedown=move |ev: MouseEvent| ev.prevent_default()
            on:click=move |_| {
                click_editor.redo();
            }
        >
            "↷"
        </button>
    }
}
