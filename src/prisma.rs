#[cfg(feature = "ssr")]
pub type ArcPrisma = std::sync::Arc<prisma_client::db::PrismaClient>;

#[cfg(feature = "ssr")]
use leptos::{use_context, Scope, ServerFnError};
#[cfg(feature = "ssr")]
pub fn use_prisma(cx: Scope) -> Result<ArcPrisma, ServerFnError> {
    use_context::<ArcPrisma>(cx)
        .ok_or("Prisma missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
