//! Slate document model and pure helpers.
//!
//! The model is shared by the SSR build (which reads the JSON from the database)
//! and the hydration build (which receives it back from a server function), so it
//! must not be gated behind the `ssr` feature.
//!
//! Parsing is deliberately tolerant: a node with an unknown `type` is kept as
//! [`SlateBlock::Unknown`] and its children are still rendered, so a document
//! produced by a newer editor cannot blank out a page.

use serde::{Deserialize, Serialize};

pub type SlateBlocks = Vec<SlateBlock>;

/// A document node.
///
/// Known editor blocks keep their typed shape, text nodes stay flat, and
/// anything else falls back to [`Unknown`] so its children survive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlateBlock {
    Known(KnownBlock),
    Text(Text),
    Unknown(Unknown),
}

/// Blocks the site knows how to render semantically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum KnownBlock {
    #[serde(rename = "p")]
    P(Container),
    #[serde(rename = "h1")]
    H1(Container),
    #[serde(rename = "h2")]
    H2(Container),
    #[serde(rename = "h3")]
    H3(Container),
    #[serde(rename = "h4")]
    H4(Container),
    #[serde(rename = "h5")]
    H5(Container),
    #[serde(rename = "h6")]
    H6(Container),
    #[serde(rename = "blockquote")]
    Blockquote(Container),
    #[serde(rename = "ul")]
    Ul(Container),
    #[serde(rename = "ol")]
    Ol(Container),
    #[serde(rename = "li")]
    Li(Container),
    #[serde(rename = "a")]
    A(A),
    #[serde(rename = "img")]
    Img(Img),
    #[serde(rename = "inline-image")]
    InlineImage(InlineImage),
    #[serde(rename = "pre")]
    Pre(Container),
    /// Code blocks from rich paste are stored as a top-level `code` element.
    #[serde(rename = "code")]
    Code(Container),
    #[serde(rename = "span")]
    Span(Container),
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "table")]
    Table(Container),
    #[serde(rename = "thead")]
    Thead(Container),
    #[serde(rename = "tbody")]
    Tbody(Container),
    #[serde(rename = "tfoot")]
    Tfoot(Container),
    #[serde(rename = "tr")]
    Tr(Container),
    #[serde(rename = "th")]
    Th(Container),
    #[serde(rename = "td")]
    Td(Container),
}

/// An element with children and no site-specific semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Container {
    pub children: Vec<SlateBlock>,
}

/// A node with a `type` the site does not know.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unknown {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(default)]
    pub children: Vec<SlateBlock>,
}

/// Parses the JSON stored in the `content.json` column.
pub fn parse_content_json(content_json: &str) -> Result<SlateBlocks, serde_json::Error> {
    serde_json::from_str(content_json)
}

/// Every content-image id referenced by a document, in document order.
///
/// Walks the raw JSON so nested images (lists, tables, custom blocks) are
/// found even when their surrounding node type is unknown to the site.
pub fn image_ids(content_json: &str) -> Vec<String> {
    fn walk(value: &serde_json::Value, out: &mut Vec<String>) {
        match value {
            serde_json::Value::Array(items) => {
                for item in items {
                    walk(item, out);
                }
            }
            serde_json::Value::Object(map) => {
                if map.get("type").and_then(|value| value.as_str()) == Some("img")
                    && let Some(id) = map.get("id").and_then(|value| value.as_str())
                    && !id.is_empty()
                {
                    out.push(id.to_string());
                }
                if let Some(children) = map.get("children") {
                    walk(children, out);
                }
            }
            _ => {}
        }
    }

    let Ok(value) = serde_json::from_str::<serde_json::Value>(content_json) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    walk(&value, &mut out);
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkKind {
    /// Same-site link: can be rendered with `leptos_router::<A>`.
    Internal,
    /// Absolute http(s) link or a non-navigational scheme like `mailto:`.
    External,
    /// A scheme that can execute code (or an unparsable href): do not render a link.
    Unsafe,
}

/// Classifies a link from the editor before it is rendered as an `<a>`.
pub fn link_kind(href: &str) -> LinkKind {
    let href = href.trim();
    if href.is_empty() {
        return LinkKind::Unsafe;
    }
    let lower = href.to_ascii_lowercase();
    if lower.starts_with("javascript:")
        || lower.starts_with("data:")
        || lower.starts_with("vbscript:")
    {
        LinkKind::Unsafe
    } else if href.starts_with("//")
        || lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("mailto:")
        || lower.starts_with("tel:")
    {
        LinkKind::External
    } else if href.starts_with('/')
        || href.starts_with('#')
        || href.starts_with('?')
        || !href.contains(':')
    {
        LinkKind::Internal
    } else {
        LinkKind::Unsafe
    }
}

/// Returns the href when it is safe to render, otherwise `None`.
pub fn safe_image_src(src: &str) -> Option<&str> {
    let src = src.trim();
    if src.is_empty() {
        return None;
    }
    let lower = src.to_ascii_lowercase();
    if lower.starts_with("data:image/") {
        return Some(src);
    }
    if lower.starts_with("data:")
        || lower.starts_with("javascript:")
        || lower.starts_with("vbscript:")
    {
        return None;
    }
    if src.starts_with("//")
        || lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("blob:")
        || src.starts_with('/')
        || src.starts_with("./")
        || src.starts_with("../")
        || !src.contains(':')
    {
        Some(src)
    } else {
        None
    }
}

/// Only allow simple CSS colour values in `color` / `background` marks.
pub fn safe_css_color(value: &str) -> Option<&str> {
    let value = value.trim();
    if value.is_empty() || value.len() > 64 {
        return None;
    }
    let lower = value.to_ascii_lowercase();
    if lower.contains("url") || lower.contains("expression") {
        return None;
    }
    value
        .chars()
        .all(|ch| {
            ch.is_ascii_alphanumeric()
                || matches!(ch, '#' | '(' | ')' | ',' | '.' | '%' | ' ' | '-')
        })
        .then_some(value)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A {
    /// Flat `href` used by the editor's default link shape.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Nested `attributes.href`, the shape this site stores.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AProps>,
    pub children: Vec<SlateBlock>,
}

impl A {
    pub fn href(&self) -> &str {
        self.attributes
            .as_ref()
            .map(|attributes| attributes.href.as_str())
            .or(self.href.as_deref())
            .unwrap_or("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AProps {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Img {
    /// Content-image id; empty for images pasted from an external source.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// External source, used when `id` is empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default)]
    pub alt: String,
    #[serde(default)]
    pub caption: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineImage {
    #[serde(default)]
    pub src: String,
    #[serde(default)]
    pub alt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strong: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub em: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superscript: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscript: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_text_and_blocks() {
        let blocks = parse_content_json(
            r#"[{"text":"hello"},{"type":"p","children":[{"text":"body","strong":true}]}]"#,
        )
        .expect("valid slate json");
        assert_eq!(blocks.len(), 2);
        assert!(matches!(blocks[0], SlateBlock::Text(_)));
        let SlateBlock::Known(KnownBlock::P(p)) = &blocks[1] else {
            panic!("expected a paragraph");
        };
        assert!(matches!(p.children[0], SlateBlock::Text(_)));
    }

    #[test]
    fn parses_an_image_and_ignores_editor_children() {
        // The editor stores `children` on image nodes; the model has no such field.
        let blocks = parse_content_json(
            r#"[{"type":"img","id":"abc","alt":"a","caption":"c","children":[{"text":""}]}]"#,
        )
        .expect("valid slate json");
        let SlateBlock::Known(KnownBlock::Img(img)) = &blocks[0] else {
            panic!("expected an image");
        };
        assert_eq!(img.id, "abc");
    }

    #[test]
    fn parses_external_images_and_tables() {
        let blocks = parse_content_json(
            r#"[
                {"type":"img","src":"https://example.com/a.png","alt":"a","children":[{"text":""}]},
                {"type":"table","children":[
                    {"type":"tbody","children":[
                        {"type":"tr","children":[
                            {"type":"td","children":[{"text":"cell"}]}
                        ]}
                    ]}
                ]}
            ]"#,
        )
        .expect("valid slate json");
        let SlateBlock::Known(KnownBlock::Img(img)) = &blocks[0] else {
            panic!("expected an image");
        };
        assert_eq!(img.src.as_deref(), Some("https://example.com/a.png"));
        assert!(matches!(
            &blocks[1],
            SlateBlock::Known(KnownBlock::Table(_))
        ));
    }

    #[test]
    fn unknown_nodes_keep_their_children() {
        let blocks = parse_content_json(
            r#"[{"type":"callout","children":[{"type":"p","children":[{"text":"inside"}]}]}]"#,
        )
        .expect("valid slate json");
        let SlateBlock::Unknown(unknown) = &blocks[0] else {
            panic!("expected an unknown block");
        };
        assert_eq!(unknown.node_type.as_deref(), Some("callout"));
        assert_eq!(unknown.children.len(), 1);
    }

    #[test]
    fn accepts_both_link_shapes() {
        let blocks = parse_content_json(
            r#"[
                {"type":"a","attributes":{"href":"/a"},"children":[{"text":"a"}]},
                {"type":"a","href":"/b","children":[{"text":"b"}]}
            ]"#,
        )
        .expect("valid slate json");
        let SlateBlock::Known(KnownBlock::A(nested)) = &blocks[0] else {
            panic!("expected a link");
        };
        assert_eq!(nested.href(), "/a");
        let SlateBlock::Known(KnownBlock::A(flat)) = &blocks[1] else {
            panic!("expected a link");
        };
        assert_eq!(flat.href(), "/b");
    }

    #[test]
    fn collects_image_ids_recursively() {
        let ids = image_ids(
            r#"[
                {"type":"img","id":"one","children":[{"text":""}]},
                {"type":"table","children":[
                    {"type":"tbody","children":[
                        {"type":"tr","children":[
                            {"type":"td","children":[{"type":"img","id":"two"}]}
                        ]}
                    ]}
                ]},
                {"type":"img","src":"https://example.com/x.png"},
                {"type":"img","id":"","src":"/cdn/x"}
            ]"#,
        );
        assert_eq!(ids, vec!["one", "two"]);
    }

    #[test]
    fn link_kinds() {
        assert_eq!(link_kind("/blog/post"), LinkKind::Internal);
        assert_eq!(link_kind("blog/post"), LinkKind::Internal);
        assert_eq!(link_kind("#section"), LinkKind::Internal);
        assert_eq!(link_kind("?page=2"), LinkKind::Internal);
        assert_eq!(link_kind("https://example.com/a"), LinkKind::External);
        assert_eq!(link_kind("//example.com/a"), LinkKind::External);
        assert_eq!(link_kind("HTTP://Example.com"), LinkKind::External);
        assert_eq!(link_kind("mailto:a@b.c"), LinkKind::External);
        assert_eq!(link_kind("tel:+123"), LinkKind::External);
        assert_eq!(link_kind("javascript:alert(1)"), LinkKind::Unsafe);
        assert_eq!(link_kind(" JavaScript:alert(1)"), LinkKind::Unsafe);
        assert_eq!(link_kind("data:text/html,x"), LinkKind::Unsafe);
        assert_eq!(link_kind("vbscript:msgbox(1)"), LinkKind::Unsafe);
        // Unknown schemes are not rendered as links either.
        assert_eq!(link_kind("java\tscript:alert(1)"), LinkKind::Unsafe);
        assert_eq!(link_kind("ftp://example.com/a"), LinkKind::Unsafe);
        assert_eq!(link_kind(""), LinkKind::Unsafe);
        assert_eq!(link_kind("   "), LinkKind::Unsafe);
    }

    #[test]
    fn image_sources_and_colours_are_sanitized() {
        assert_eq!(safe_image_src("/cdn/a_l"), Some("/cdn/a_l"));
        assert_eq!(
            safe_image_src("https://example.com/a.png"),
            Some("https://example.com/a.png")
        );
        assert_eq!(
            safe_image_src("data:image/png;base64,abc"),
            Some("data:image/png;base64,abc")
        );
        assert_eq!(safe_image_src("data:text/html,x"), None);
        assert_eq!(safe_image_src("javascript:alert(1)"), None);
        assert_eq!(safe_image_src("ftp://example.com/a"), None);

        assert_eq!(safe_css_color("#ff0000"), Some("#ff0000"));
        assert_eq!(
            safe_css_color("rgba(0, 0, 0, 0.5)"),
            Some("rgba(0, 0, 0, 0.5)")
        );
        assert_eq!(safe_css_color("red; background: url(x)"), None);
        assert_eq!(safe_css_color("url(x)"), None);
    }
}
