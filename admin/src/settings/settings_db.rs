use super::SettingsCx;

pub async fn settings_db(pool: cornucopia::deadpool_postgres::Pool) -> SettingsCx {
    use cornucopia::queries;
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            tracing::error!("settings: failed to get db connection: {e}");
            return SettingsCx::default();
        }
    };
    let settings = match queries::settings::settings().bind(&client).opt().await {
        Ok(Some(settings)) => settings,
        Ok(None) => {
            tracing::error!("settings: no Settings record found (run `cli settings-init`)");
            return SettingsCx::default();
        }
        Err(e) => {
            tracing::error!("settings: failed to load settings: {e}");
            return SettingsCx::default();
        }
    };
    SettingsCx {
        site_url: settings.site_url,
    }
}
