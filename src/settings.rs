use leptos::{use_context, Scope};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsCx {
    pub site_url: String,
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

pub fn use_settings(cx: Scope) -> SettingsCx {
    let settings = use_context::<SettingsCx>(cx).expect("SettingsCx");
    settings
}
pub fn use_site_url(cx: Scope) -> String {
    let settings = use_settings(cx);
    settings.site_url
}

#[cfg(feature = "ssr")]
pub async fn use_settins_db(cx: Scope) -> SettingsCx {
    let prisma_client = crate::prisma::use_prisma(cx).unwrap();
    settins_db(prisma_client).await
}

#[cfg(feature = "ssr")]
pub async fn settins_db(prisma_client: crate::prisma::ArcPrisma) -> SettingsCx {
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
