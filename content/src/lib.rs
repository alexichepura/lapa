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
        format!("<a href={}>{}</a>", self.attributes.href, children_html)
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
        format!(
            "<figure><img src=\"/cdn/{}_lg\" alt=\"{}\" srcset=\"{srcset}\" sizes=\"{sizes}\" loading=\"lazy\"/><figcaption>{}</figcaption></figure>",
            self.id, self.alt, self.caption
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
