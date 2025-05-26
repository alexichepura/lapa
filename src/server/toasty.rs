use toasty::{stmt::Id, Db};

#[derive(Debug, toasty::Model)]
// #[table = "Settings"]
pub struct Settings {
    #[key]
    #[auto]
    id: Id<Self>,

    pub home_text: String,
    pub site_url: String,
    pub hero_height: i64,
    pub hero_width: i64,
    pub thumb_height: i64,
    pub thumb_width: i64,
}

#[derive(Debug, toasty::Model)]
pub struct Post {
    #[key]
    #[auto]
    pub id: Id<Self>,

    #[unique]
    pub slug: String,

    pub title: String,
    pub description: String,
    pub text: String,

    #[has_many]
    pub images: toasty::HasMany<Image>,
}

#[derive(Debug, toasty::Model)]
pub struct Image {
    #[key]
    #[auto]
    pub id: Id<Self>,
    // created_at DateTime @default(now())
    pub is_hero: i64,
    pub alt: String,
    pub ext: String,
    pub order: i64,

    pub post_id: Id<Post>,

    #[belongs_to(key = post_id, references = id)]
    post: toasty::BelongsTo<Post>,
}

pub async fn dbuild() -> toasty::Result<Db> {
    // let db_url = std::env::var("TOASTY_CONNECTION_URL")
    //     .as_deref()
    //     .unwrap_or("sqlite::memory:");
    let db_url = std::env::var("TOASTY_CONNECTION_URL");
    tracing::trace!("{:?}", db_url);
    let db_url = db_url.unwrap();
    let db = Db::builder()
        .register::<Settings>()
        .register::<Post>()
        .register::<Image>()
        .connect(&db_url)
        .await?;
    // For now, reset!s
    // db.reset_db().await?;
    Ok(db)
}
