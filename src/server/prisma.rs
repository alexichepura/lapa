use leptos::{use_context, ServerFnError};
use prisma_client::db;
use std::sync::Arc;

pub type ArcPrisma = Arc<db::PrismaClient>;

pub async fn init_prisma_client() -> ArcPrisma {
    let client = if let Ok(db_url) = std::env::var("DATABASE_URL") {
        db::new_client_with_url(db_url.as_str()).await
    } else {
        db::new_client().await
    };
    let prisma_client = Arc::new(client.unwrap());
    #[cfg(debug)]
    prisma_client._db_push(false).await.unwrap();
    prisma_client
}

pub fn use_prisma() -> Result<ArcPrisma, ServerFnError> {
    use_context::<ArcPrisma>()
        .ok_or("Prisma missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
