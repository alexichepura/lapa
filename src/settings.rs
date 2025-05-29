use leptos::prelude::use_context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsCx {
    pub site_url: String,
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

pub fn use_settings() -> SettingsCx {
    let settings = use_context::<SettingsCx>().expect("SettingsCx");
    settings
}
pub fn use_site_url() -> String {
    let settings = use_settings();
    settings.site_url
}

// #[cfg(feature = "ssr")]
// pub async fn use_settins_db() -> SettingsCx {
//     let prisma_client = crate::server::use_prisma().unwrap();
//     settins_db(prisma_client).await
// }

#[cfg(feature = "ssr")]
pub async fn settings_prisma(prisma_client: crate::server::ArcPrisma) -> SettingsCx {
    use prisma_client::db;
    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({
            site_url
            hero_height
            hero_width
            thumb_height
            thumb_width
        }))
        .exec()
        .await
        .unwrap();
    let settings = settings.unwrap();
    let settings = SettingsCx {
        site_url: settings.site_url,
        hero_height: settings.hero_height,
        hero_width: settings.hero_width,
        thumb_height: settings.thumb_height,
        thumb_width: settings.thumb_width,
    };
    settings
}
#[cfg(feature = "ssr")]
pub async fn settings_db(pool: clorinde::deadpool_postgres::Pool) -> SettingsCx {
    use clorinde::queries;
    let client = pool.get().await.unwrap();
    let settings = queries::settings::settings().bind(&client).opt().await.unwrap();
    dbg!(&settings);
    let settings = settings.unwrap();

    let settings = SettingsCx {
        site_url: settings.site_url,
        hero_height: settings.hero_height,
        hero_width: settings.hero_width,
        thumb_height: settings.thumb_height,
        thumb_width: settings.thumb_width,
    };
    settings
}
