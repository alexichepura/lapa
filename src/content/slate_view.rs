use content::{CdnImageSize, SlateBlock, SlateBlocks, Text};
use leptos::prelude::*;
use leptos_router::components::A as RouterLink;

/// Renders a Slate document as Leptos views.
///
/// Markup, text and attribute values are escaped by `view!`, so the content JSON
/// can never break out of a tag or an attribute.
#[component]
pub fn SlateContent(blocks: SlateBlocks) -> AnyView {
    blocks.into_iter().map(block_view).collect_view().into_any()
}

fn block_view(block: SlateBlock) -> AnyView {
    match block {
        SlateBlock::P(p) => view! { <p><SlateContent blocks=p.children /></p> }.into_any(),
        SlateBlock::H1(h) => view! { <h1><SlateContent blocks=h.children /></h1> }.into_any(),
        SlateBlock::H2(h) => view! { <h2><SlateContent blocks=h.children /></h2> }.into_any(),
        SlateBlock::H3(h) => view! { <h3><SlateContent blocks=h.children /></h3> }.into_any(),
        SlateBlock::H4(h) => view! { <h4><SlateContent blocks=h.children /></h4> }.into_any(),
        SlateBlock::Blockquote(b) => {
            view! { <blockquote><SlateContent blocks=b.children /></blockquote> }.into_any()
        }
        SlateBlock::Ul(ul) => view! { <ul><SlateContent blocks=ul.children /></ul> }.into_any(),
        SlateBlock::Ol(ol) => view! { <ol><SlateContent blocks=ol.children /></ol> }.into_any(),
        SlateBlock::LI(li) => view! { <li><SlateContent blocks=li.children /></li> }.into_any(),
        SlateBlock::A(a) => link_view(a),
        SlateBlock::Img(img) => image_view(img),
        SlateBlock::Text(text) => text_view(text),
    }
}

fn link_view(link: content::A) -> AnyView {
    let content::A {
        attributes,
        children,
    } = link;
    let href = attributes.href;
    match content::link_kind(&href) {
        // `#fragment` links must scroll, not go through the router.
        content::LinkKind::Internal if !href.starts_with('#') => {
            view! { <RouterLink href=href><SlateContent blocks=children /></RouterLink> }.into_any()
        }
        content::LinkKind::Internal => {
            view! { <a href=href><SlateContent blocks=children /></a> }.into_any()
        }
        content::LinkKind::External => view! {
            <a href=href rel="noopener noreferrer"><SlateContent blocks=children /></a>
        }
        .into_any(),
        // Never render an executable href: keep the text, drop the link.
        content::LinkKind::Unsafe => {
            view! { <span><SlateContent blocks=children /></span> }.into_any()
        }
    }
}

fn image_view(img: content::Img) -> AnyView {
    // https://observablehq.com/@eeeps/w-descriptors-and-sizes-under-the-hood
    let srcset = CdnImageSize::VALUES
        .map(|size| content::cdn_img(&img.id, size))
        .join(", ");
    let sizes = format!(
        "(max-width: {}px) {}px, 100vw",
        CdnImageSize::S.to_width(),
        CdnImageSize::S.to_width(),
    );
    let src = content::cdn_img_src(&img.id, CdnImageSize::L);
    view! {
        <figure>
            <img src=src alt=img.alt.clone() srcset=srcset sizes=sizes loading="lazy" />
            {(!img.caption.is_empty())
                .then(|| view! { <figcaption>{img.caption}</figcaption> })}
        </figure>
    }
    .into_any()
}

fn text_view(text: Text) -> AnyView {
    let Text {
        text,
        strong,
        em,
        u,
        code,
    } = text;

    let mut view: AnyView = text.into_any();
    if code.unwrap_or(false) {
        view = view! { <code>{view}</code> }.into_any();
    }
    if u.unwrap_or(false) {
        view = view! { <u>{view}</u> }.into_any();
    }
    if em.unwrap_or(false) {
        view = view! { <em>{view}</em> }.into_any();
    }
    if strong.unwrap_or(false) {
        view = view! { <strong>{view}</strong> }.into_any();
    }
    view
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    fn render(json: &str) -> String {
        let blocks = content::parse_content_json(json).expect("valid slate json");
        view! { <SlateContent blocks=blocks /> }
            .to_html()
            .replace("<!>", "")
    }

    #[test]
    fn text_and_marks_are_rendered_and_escaped() {
        let html = render(
            r#"[{"type":"p","children":[
                {"text":"a <script> & b"},
                {"text":"bold","strong":true},
                {"text":"em","em":true},
                {"text":"both","strong":true,"em":true},
                {"text":"code","code":true},
                {"text":"under","u":true},
                {"text":"plain","strong":false}
            ]}]"#,
        );
        assert_eq!(
            html,
            "<p>a &lt;script&gt; &amp; b<strong>bold</strong><em>em</em>\
             <strong><em>both</em></strong><code>code</code><u>under</u>plain</p>"
        );
    }

    #[test]
    fn image_attributes_and_caption_are_escaped() {
        let html = render(
            r#"[{"type":"img","id":"abc","alt":"say \"hi\" <b>","caption":"cap & <i>more</i>"}]"#,
        );
        assert!(html.contains(r#"<img src="/cdn/abc_l""#), "{html}");
        assert!(
            html.contains(r#"alt="say &quot;hi&quot; &lt;b&gt;""#),
            "{html}"
        );
        assert!(
            html.contains("<figcaption>cap &amp; &lt;i&gt;more&lt;/i&gt;</figcaption>"),
            "{html}"
        );
        assert!(html.contains(r#"loading="lazy""#), "{html}");
        assert!(html.contains(r#"/cdn/abc_s 480w"#), "{html}");
    }

    #[test]
    fn empty_caption_does_not_render_figcaption() {
        let html = render(r#"[{"type":"img","id":"abc","alt":"a","caption":""}]"#);
        assert!(!html.contains("figcaption"), "{html}");
    }

    #[test]
    fn unsafe_links_are_not_rendered_as_anchors() {
        let html = render(
            r#"[{"type":"p","children":[
                {"type":"a","attributes":{"href":"javascript:alert(1)"},"children":[{"text":"click"}]}
            ]}]"#,
        );
        assert!(!html.contains("<a"), "{html}");
        assert!(!html.contains("javascript:"), "{html}");
        assert!(html.contains("click"), "{html}");
    }

    #[test]
    fn fragment_and_external_links_are_plain_anchors() {
        let html = render(
            r##"[{"type":"p","children":[
                {"type":"a","attributes":{"href":"#section"},"children":[{"text":"go"}]},
                {"type":"a","attributes":{"href":"https://example.com"},"children":[{"text":"out"}]}
            ]}]"##,
        );
        assert!(html.contains(r##"<a href="#section">go</a>"##), "{html}");
        assert!(
            html.contains(r#"<a href="https://example.com" rel="noopener noreferrer">out</a>"#),
            "{html}"
        );
    }

    #[test]
    fn internal_links_use_the_router() {
        let blocks = content::parse_content_json(
            r#"[{"type":"p","children":[
                {"type":"a","attributes":{"href":"/product/demo"},"children":[{"text":"product"}]}
            ]}]"#,
        )
        .expect("valid slate json");

        let owner = Owner::new();
        let html = owner.with(|| {
            provide_context(leptos_router::location::RequestUrl::new("/"));
            view! { <leptos_router::components::Router><SlateContent blocks=blocks /></leptos_router::components::Router> }
                .to_html()
                .replace("<!>", "")
        });

        assert!(html.contains(r#"href="/product/demo""#), "{html}");
        assert!(html.contains("product"), "{html}");
    }

    #[test]
    fn lists_and_block_tree_are_not_flattened() {
        let html = render(
            r#"[{"type":"blockquote","children":[
                {"type":"ul","children":[
                    {"type":"li","children":[{"text":"one"}]},
                    {"type":"li","children":[{"text":"two"}]}
                ]}
            ]}]"#,
        );
        assert_eq!(
            html,
            "<blockquote><ul><li>one</li><li>two</li></ul></blockquote>"
        );
    }
}
