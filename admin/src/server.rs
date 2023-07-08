#[cfg(feature = "ssr")]
pub fn published_filter() -> prisma_client::db::read_filters::DateTimeFilter {
    use prisma_client::db::read_filters::DateTimeFilter;
    use prisma_client_rust::chrono::Utc;

    DateTimeFilter::Lt(Utc::now().fixed_offset())
}

#[cfg(feature = "ssr")]
pub fn post_where_published() -> prisma_client::db::post::WhereParam {
    prisma_client::db::post::WhereParam::PublishedAt(published_filter())
}
