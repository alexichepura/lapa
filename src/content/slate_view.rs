use content::{CdnImageSize, KnownBlock, SlateBlock, SlateBlocks, Text};
use leptos::prelude::*;
use leptos_router::components::A as RouterLink;

/// Renders a Slate document as Leptos views.
///
/// Markup, text and attribute values are escaped by `view!`, so the content JSON
/// can never break out of a tag or an attribute. Unknown nodes are transparent:
/// their children are rendered without a wrapper.
#[component]
pub fn SlateContent(blocks: SlateBlocks) -> AnyView {
    blocks.into_iter().map(block_view).collect_view().into_any()
}

fn block_view(block: SlateBlock) -> AnyView {
    match block {
        SlateBlock::Known(known) => known_view(known),
        SlateBlock::Text(text) => text_view(text),
        SlateBlock::Unknown(unknown) => {
            view! { <SlateContent blocks=unknown.children /> }.into_any()
        }
    }
}

fn known_view(block: KnownBlock) -> AnyView {
    match block {
        KnownBlock::P(c) => view! { <p><SlateContent blocks=c.children /></p> }.into_any(),
        KnownBlock::H1(c) => view! { <h1><SlateContent blocks=c.children /></h1> }.into_any(),
        KnownBlock::H2(c) => view! { <h2><SlateContent blocks=c.children /></h2> }.into_any(),
        KnownBlock::H3(c) => view! { <h3><SlateContent blocks=c.children /></h3> }.into_any(),
        KnownBlock::H4(c) => view! { <h4><SlateContent blocks=c.children /></h4> }.into_any(),
        KnownBlock::H5(c) => view! { <h5><SlateContent blocks=c.children /></h5> }.into_any(),
        KnownBlock::H6(c) => view! { <h6><SlateContent blocks=c.children /></h6> }.into_any(),
        KnownBlock::Blockquote(c) => {
            view! { <blockquote><SlateContent blocks=c.children /></blockquote> }.into_any()
        }
        KnownBlock::Ul(c) => view! { <ul><SlateContent blocks=c.children /></ul> }.into_any(),
        KnownBlock::Ol(c) => view! { <ol><SlateContent blocks=c.children /></ol> }.into_any(),
        KnownBlock::Li(c) => view! { <li><SlateContent blocks=c.children /></li> }.into_any(),
        KnownBlock::A(a) => link_view(a),
        KnownBlock::Img(img) => image_view(img),
        KnownBlock::InlineImage(img) => inline_image_view(img),
        KnownBlock::Pre(c) => view! { <pre><SlateContent blocks=c.children /></pre> }.into_any(),
        // Rich paste stores fenced code as a top-level `code` element.
        KnownBlock::Code(c) => view! {
            <pre><code><SlateContent blocks=c.children /></code></pre>
        }
        .into_any(),
        KnownBlock::Span(c) => view! { <span><SlateContent blocks=c.children /></span> }.into_any(),
        KnownBlock::Hr => view! { <hr /> }.into_any(),
        KnownBlock::Table(c) => {
            view! { <table><SlateContent blocks=c.children /></table> }.into_any()
        }
        KnownBlock::Thead(c) => {
            view! { <thead><SlateContent blocks=c.children /></thead> }.into_any()
        }
        KnownBlock::Tbody(c) => {
            view! { <tbody><SlateContent blocks=c.children /></tbody> }.into_any()
        }
        KnownBlock::Tfoot(c) => {
            view! { <tfoot><SlateContent blocks=c.children /></tfoot> }.into_any()
        }
        KnownBlock::Tr(c) => view! { <tr><SlateContent blocks=c.children /></tr> }.into_any(),
        KnownBlock::Th(c) => view! { <th><SlateContent blocks=c.children /></th> }.into_any(),
        KnownBlock::Td(c) => view! { <td><SlateContent blocks=c.children /></td> }.into_any(),
    }
}

fn link_view(link: content::A) -> AnyView {
    let href = link.href().to_string();
    let children = link.children;
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
    let alt = img.alt;
    let caption = img.caption;

    if !img.id.is_empty() {
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
        return view! {
            <figure>
                <img src=src alt=alt.clone() srcset=srcset sizes=sizes loading="lazy" />
                {(!caption.is_empty())
                    .then(|| view! { <figcaption>{caption}</figcaption> })}
            </figure>
        }
        .into_any();
    }

    let Some(src) = img.src.as_deref().and_then(content::safe_image_src) else {
        return ().into_any();
    };
    let src = src.to_string();
    view! {
        <figure>
            <img src=src alt=alt.clone() loading="lazy" />
            {(!caption.is_empty()).then(|| view! { <figcaption>{caption}</figcaption> })}
        </figure>
    }
    .into_any()
}

fn inline_image_view(img: content::InlineImage) -> AnyView {
    let Some(src) = content::safe_image_src(&img.src) else {
        return ().into_any();
    };
    let src = src.to_string();
    let alt = img.alt;
    view! { <img src=src alt=alt loading="lazy" /> }.into_any()
}

fn text_view(text: Text) -> AnyView {
    let Text {
        text,
        strong,
        em,
        u,
        code,
        strikethrough,
        superscript,
        subscript,
        color,
        background,
    } = text;

    let mut view: AnyView = text.into_any();
    if code.unwrap_or(false) {
        view = view! { <code>{view}</code> }.into_any();
    }
    if u.unwrap_or(false) {
        view = view! { <u>{view}</u> }.into_any();
    }
    if strikethrough.unwrap_or(false) {
        view = view! { <s>{view}</s> }.into_any();
    }
    if em.unwrap_or(false) {
        view = view! { <em>{view}</em> }.into_any();
    }
    if strong.unwrap_or(false) {
        view = view! { <strong>{view}</strong> }.into_any();
    }
    if superscript.unwrap_or(false) {
        view = view! { <sup>{view}</sup> }.into_any();
    } else if subscript.unwrap_or(false) {
        view = view! { <sub>{view}</sub> }.into_any();
    }

    let mut styles: Vec<String> = Vec::new();
    if let Some(color) = color.as_deref().and_then(content::safe_css_color) {
        styles.push(format!("color:{color}"));
    }
    if let Some(background) = background.as_deref().and_then(content::safe_css_color) {
        styles.push(format!("background:{background}"));
    }
    if !styles.is_empty() {
        let style = styles.join(";");
        view = view! { <span style=style>{view}</span> }.into_any();
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
    fn extended_marks_are_rendered() {
        let html = render(
            r##"[{"type":"p","children":[
                {"text":"gone","strikethrough":true},
                {"text":"up","superscript":true},
                {"text":"down","subscript":true},
                {"text":"red","color":"#ff0000"},
                {"text":"bad","color":"red; background: url(x)"}
            ]}]"##,
        );
        assert!(html.contains("<s>gone</s>"), "{html}");
        assert!(html.contains("<sup>up</sup>"), "{html}");
        assert!(html.contains("<sub>down</sub>"), "{html}");
        assert!(
            html.contains(r##"<span style="color:#ff0000;">red</span>"##),
            "{html}"
        );
        assert!(!html.contains("url(x)"), "{html}");
        assert!(html.contains("bad"), "{html}");
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
    fn external_images_render_with_a_safe_src() {
        let html = render(
            r#"[{"type":"img","src":"https://example.com/a.png","alt":"a","caption":"cap"}]"#,
        );
        assert!(
            html.contains(r#"<img src="https://example.com/a.png" alt="a" loading="lazy""#),
            "{html}"
        );
        assert!(html.contains("<figcaption>cap</figcaption>"), "{html}");

        let unsafe_html = render(r#"[{"type":"img","src":"javascript:alert(1)","alt":"a"}]"#);
        assert!(!unsafe_html.contains("<img"), "{unsafe_html}");
        assert!(!unsafe_html.contains("javascript:"), "{unsafe_html}");
    }

    #[test]
    fn inline_images_render_inside_paragraphs() {
        let html = render(
            r#"[{"type":"p","children":[
                {"text":"before "},
                {"type":"inline-image","src":"/cdn/x_s","alt":"x"},
                {"text":" after"}
            ]}]"#,
        );
        assert!(html.contains(r#"<img src="/cdn/x_s" alt="x""#), "{html}");
        assert!(html.contains("before "), "{html}");
    }

    #[test]
    fn tables_are_rendered_with_semantic_tags() {
        let html = render(
            r#"[{"type":"table","children":[
                {"type":"thead","children":[
                    {"type":"tr","children":[
                        {"type":"th","children":[{"text":"H"}]}
                    ]}
                ]},
                {"type":"tbody","children":[
                    {"type":"tr","children":[
                        {"type":"td","children":[{"text":"cell"}]}
                    ]}
                ]}
            ]}]"#,
        );
        assert_eq!(
            html,
            "<table><thead><tr><th>H</th></tr></thead>\
             <tbody><tr><td>cell</td></tr></tbody></table>"
        );
    }

    #[test]
    fn code_blocks_and_rules_are_rendered() {
        let html = render(r#"[{"type":"code","children":[{"text":"let x = 1;"}]},{"type":"hr"}]"#);
        assert_eq!(html, "<pre><code>let x = 1;</code></pre><hr>");
    }

    #[test]
    fn unknown_blocks_are_transparent() {
        let html = render(
            r#"[{"type":"callout","tone":"info","children":[
                {"type":"p","children":[{"text":"inside"}]}
            ]}]"#,
        );
        assert_eq!(html, "<p>inside</p>");
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
    fn flat_and_nested_links_are_supported() {
        let html = render(
            r##"[{"type":"p","children":[
                {"type":"a","attributes":{"href":"#section"},"children":[{"text":"go"}]},
                {"type":"a","href":"https://example.com","children":[{"text":"out"}]}
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
