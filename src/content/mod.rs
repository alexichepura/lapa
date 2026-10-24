use content::SlateBlocks;

mod slate_view;
pub use slate_view::SlateContent;

/// Parses the Slate JSON from the database, logs malformed content and returns
/// an empty document so a broken row cannot break the page.
pub fn content_blocks(content_json: &str) -> SlateBlocks {
    match content::parse_content_json(content_json) {
        Ok(blocks) => blocks,
        Err(e) => {
            tracing::error!("content json parse error: {e}");
            SlateBlocks::new()
        }
    }
}
