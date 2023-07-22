use axum::extract::FromRef;
use axum::extract::State;
use http::StatusCode;
use leptos::LeptosOptions;
use prisma_client::db::PrismaClient;
use std::sync::Arc;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub prisma_client: Arc<PrismaClient>,
}

pub async fn robots_txt(State(app_state): State<AppState>) -> Result<String, (StatusCode, String)> {
    use prisma_client::db;
    let prisma_client = app_state.prisma_client;

    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({ robots_txt }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error".to_string(),
            )
        })?;
    let settings = settings.unwrap();
    Ok(settings.robots_txt)
}
