use super::SettingsCx;

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
