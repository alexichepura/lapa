use toasty::{stmt::Id, Db};

#[derive(Debug)]
#[toasty::model]
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

#[derive(Debug)]
#[toasty::model]
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
    pub images: [Image],
}

#[derive(Debug)]
#[toasty::model]
pub struct Image {
    #[key]
    #[auto]
    pub id: Id<Self>,
    // created_at DateTime @default(now())
    // pub is_hero: bool,
    pub alt: String,
    pub ext: String,
    pub order: i64,

    #[index]
    post_id: Id<Post>,

    #[belongs_to(key = post_id, references = id)]
    post: Post,
}

pub async fn dbuild() -> toasty::Result<Db> {
    let db = Db::builder()
        .register::<Post>()
        .register::<Settings>()
        .connect(
            std::env::var("TOASTY_CONNECTION_URL")
                .as_deref()
                .unwrap_or("sqlite::memory:"),
        )
        .await?;
    // For now, reset!s
    // db.reset_db().await?;
    Ok(db)
}
