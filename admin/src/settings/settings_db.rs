use leptos::Scope;

use super::SettingsCx;

pub async fn use_settins_db(cx: Scope) -> SettingsCx {
    let prisma_client = crate::prisma::use_prisma(cx).unwrap();
    settins_db(prisma_client).await
}

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
