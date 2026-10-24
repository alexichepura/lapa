//! Slate document model and pure helpers.
//!
//! The model is shared by the SSR build (which reads the JSON from the database)
//! and the hydration build (which receives it back from a server function), so it
//! must not be gated behind the `ssr` feature.

use serde::{Deserialize, Serialize};

pub type SlateBlocks = Vec<SlateBlock>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlateBlock {
    #[serde(rename = "p")]
    P(P),
    #[serde(rename = "h1")]
    H1(H1),
    #[serde(rename = "h2")]
    H2(H2),
    #[serde(rename = "h3")]
    H3(H3),
    #[serde(rename = "h4")]
    H4(H4),
    #[serde(rename = "blockquote")]
    Blockquote(Blockquote),
    #[serde(rename = "ul")]
    Ul(Ul),
    #[serde(rename = "ol")]
    Ol(Ol),
    #[serde(rename = "li")]
    LI(Li),
    #[serde(rename = "a")]
    A(A),
    #[serde(rename = "img")]
    Img(Img),
    #[serde(untagged)]
    Text(Text),
}

/// Parses the JSON stored in the `content.json` column.
pub fn parse_content_json(content_json: &str) -> Result<SlateBlocks, serde_json::Error> {
    serde_json::from_str(content_json)
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct P {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct H1 {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct H2 {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct H3 {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct H4 {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blockquote {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ol {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ul {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Li {
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A {
    pub attributes: AProps,
    pub children: Vec<SlateBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AProps {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Img {
    pub id: String,
    pub alt: String,
    pub caption: String,
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
        let SlateBlock::P(p) = &blocks[1] else {
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
        let SlateBlock::Img(img) = &blocks[0] else {
            panic!("expected an image");
        };
        assert_eq!(img.id, "abc");
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
}
