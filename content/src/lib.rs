use serde::{Deserialize, Serialize};
mod cdn;
pub use cdn::*;

pub fn content_json_to_html(content_json: &str) -> String {
    let slate_model = serde_json::from_str::<SlateBlocks>(content_json);
    // tracing::debug!("slate_model={:?}", slate_model);
    match slate_model {
        Ok(model) => model.to_html(),
        Err(_e) => "".into(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
trait SlateToHtml {
    fn to_html(&self) -> String;
}
pub type SlateBlocks = Vec<SlateBlock>;
impl SlateToHtml for SlateBlocks {
    fn to_html(&self) -> String {
        let is_single = self.len() == 1;
        let html = self
            .into_iter()
            .map(|block| block.to_html(is_single))
            .collect::<Vec<String>>()
            .join("");
        html
    }
}

impl SlateBlock {
    fn to_html(&self, is_single: bool) -> String {
        match self {
            SlateBlock::P(p) => p.to_html(),
            SlateBlock::H1(h) => h.to_html(),
            SlateBlock::H2(h) => h.to_html(),
            SlateBlock::H3(h) => h.to_html(),
            SlateBlock::H4(h) => h.to_html(),
            SlateBlock::Blockquote(blockuote) => blockuote.to_html(),
            SlateBlock::Ul(ul) => ul.to_html(),
            SlateBlock::Ol(ol) => ol.to_html(),
            SlateBlock::LI(li) => li.to_html(),
            SlateBlock::A(a) => a.to_html(),
            SlateBlock::Img(img) => img.to_html(),
            SlateBlock::Text(text) => text.to_html(is_single),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct P {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for P {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<p>{}</p>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct H1 {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for H1 {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<h1>{}</h1>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct H2 {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for H2 {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<h2>{}</h2>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct H3 {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for H3 {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<h3>{}</h3>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct H4 {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for H4 {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<h4>{}</h4>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Blockquote {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for Blockquote {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<blockquote>{}</blockquote>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ol {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for Ol {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<ol>{}</ol>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ul {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for Ul {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<ul>{}</ul>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Li {
    children: Vec<SlateBlock>,
}
impl SlateToHtml for Li {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        format!("<li>{}</li>", children_html)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct A {
    attributes: AProps,
    children: Vec<SlateBlock>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AProps {
    href: String, // TODO consider Option<String>
}
impl SlateToHtml for A {
    fn to_html(&self) -> String {
        let children_html = self.children.to_html();
        let href = html_escape::encode_double_quoted_attribute(safe_href(&self.attributes.href));
        format!("<a href=\"{}\">{}</a>", href, children_html)
    }
}

/// Drops script-bearing URL schemes, tolerating leading whitespace and embedded
/// control characters (`java\tscript:`), so the escaped href cannot execute.
fn safe_href(href: &str) -> &str {
    let trimmed = href.trim_start();
    let normalized: String = trimmed
        .chars()
        .filter(|c| !c.is_ascii_control())
        .collect::<String>()
        .to_ascii_lowercase();
    const DANGEROUS: [&str; 3] = ["javascript:", "vbscript:", "data:"];
    if DANGEROUS
        .iter()
        .any(|scheme| normalized.starts_with(scheme))
    {
        "#"
    } else {
        trimmed
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Img {
    pub id: String,
    alt: String,
    caption: String,
}
pub fn cdn_img(id: &str, size: CdnImageSize) -> String {
    format!("/cdn/{id}_{size} {}w", size.to_width())
}
impl SlateToHtml for Img {
    fn to_html(&self) -> String {
        // https://observablehq.com/@eeeps/w-descriptors-and-sizes-under-the-hood
        let srcset = CdnImageSize::VALUES
            .map(|size| cdn_img(&self.id, size))
            .join(", ");
        let sizes = format!(
            "(max-width: {}px) {}px, 100vw",
            CdnImageSize::S.to_width(),
            CdnImageSize::S.to_width(),
        );
        let alt = html_escape::encode_double_quoted_attribute(&self.alt);
        let caption = html_escape::encode_text(&self.caption);
        let l = CdnImageSize::L;
        format!(
            "<figure><img src=\"/cdn/{}_{l}\" alt=\"{}\" srcset=\"{srcset}\" sizes=\"{sizes}\" loading=\"lazy\"/><figcaption>{}</figcaption></figure>",
            self.id, alt, caption
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    text: String,
    strong: Option<bool>,
    em: Option<bool>,
    u: Option<bool>,
    code: Option<bool>,
}
impl Text {
    fn to_html(&self, is_single: bool) -> String {
        let Text {
            text,
            strong,
            em,
            u,
            code,
        } = self;
        let text = html_escape::encode_text(&text).into_owned();

        if strong.is_some() && em.is_some() {
            format!("<strong><em>{}</em></strong>", text)
        } else if strong.is_some() {
            format!("<strong>{}</strong>", text)
        } else if em.is_some() {
            format!("<em>{}</em>", text)
        } else if u.is_some() {
            format!("<u>{}</u>", text)
        } else if code.is_some() {
            format!("<code>{}</code>", text)
        } else {
            match is_single {
                true => text.clone(),
                false => format!("<span>{}</span>", text),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_text() {
        let html = content_json_to_html(r#"[{"text":"a < b & c"}]"#);
        assert!(html.contains("a &lt; b &amp; c"));
    }

    #[test]
    fn escapes_link_href_and_blocks_script_scheme() {
        let html = content_json_to_html(
            r#"[
                {"type":"a","attributes":{"href":"javascript:alert(1)"},"children":[{"text":"x"}]},
                {"type":"a","attributes":{"href":"/ok\" onmouseover=\"alert(1)"},"children":[{"text":"y"}]}
            ]"#,
        );
        assert!(html.contains(r##"<a href="#">x</a>"##));
        assert!(!html.contains("javascript:"));
        // The injected quote must be escaped so it cannot break out of the attribute.
        assert!(!html.contains(r#"" onmouseover=""#));
        assert!(html.contains("&quot; onmouseover=&quot;"));
    }

    #[test]
    fn escapes_image_alt_and_caption() {
        let html = content_json_to_html(
            r#"[{"type":"img","id":"abc","alt":"<img src=x onerror=alert(1)>","caption":"a < b & c"}]"#,
        );
        assert!(html.contains(r#"alt="&lt;img src=x onerror=alert(1)&gt;""#));
        assert!(html.contains("<figcaption>a &lt; b &amp; c</figcaption>"));
        // The `src` fallback must use a valid CDN size (the handler only knows
        // s/m/l/xl), not the old broken `_lg`.
        assert!(html.contains(r#"src="/cdn/abc_l""#));
        assert!(!html.contains("_lg"));
    }
}
