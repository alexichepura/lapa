use async_trait::async_trait;
use axum_session::{
    DatabaseError, DatabasePool, SessionConfig, SessionLayer, SessionStore,
};
use chrono::{DateTime};
use clorinde::{deadpool_postgres::Pool, queries};

pub async fn session_layer(pool: &Pool) -> SessionLayer<SessionPool> {
    let config = SessionConfig::default()
        .with_table_name("Session")
        .with_session_name("session");

    let store = SessionStore::<SessionPool>::new(Some(pool.clone().into()), config)
        .await
        .unwrap();

    let layer = SessionLayer::new(store);
    layer
}

#[derive(Debug, Clone)]
pub struct SessionPool {
    pool: Pool,
}

impl From<Pool> for SessionPool {
    fn from(pool: Pool) -> Self {
        SessionPool { pool }
    }
}

#[async_trait]
impl DatabasePool for SessionPool {
    async fn initiate(&self, _table_name: &str) -> Result<(), DatabaseError> {
        Ok(())
    }

    async fn delete_by_expiry(&self, _table_name: &str) -> Result<Vec<String>, DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let ids = queries::session::delete_by_expiry()
            .bind(&db)
            .all()
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(ids)
    }

    async fn count(&self, _table_name: &str) -> Result<i64, DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let count = queries::session::count()
            .bind(&db)
            .one()
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        tracing::debug!("count_result={}", count);
        return Ok(count);
    }

    async fn store(
        &self,
        id: &str,
        session: &str,
        expires: i64,
        _table_name: &str,
    ) -> Result<(), DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let expires_datetime = DateTime::from_timestamp(expires, 0).unwrap_or_default();
        queries::session::store()
            .bind(&db, &id, &session, &expires_datetime.naive_utc())
            .await
            .map_err(|e| DatabaseError::GenericCreateError(e.to_string()))?;
        Ok(())
    }

    async fn load(&self, id: &str, _table_name: &str) -> Result<Option<String>, DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let id = queries::session::load()
            .bind(&db, &id)
            .opt()
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        return Ok(id);
    }

    async fn delete_one_by_id(&self, id: &str, _table_name: &str) -> Result<(), DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let _count = queries::session::delete_one_by_id()
            .bind(&db, &id)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn exists(&self, id: &str, _table_name: &str) -> Result<bool, DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let count = queries::session::exists()
            .bind(&db, &id)
            .one()
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        let exists = count > 0;
        Ok(exists)
    }

    async fn delete_all(&self, _table_name: &str) -> Result<(), DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let _count = queries::session::delete_all()
            .bind(&db)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn get_ids(&self, _table_name: &str) -> Result<Vec<String>, DatabaseError> {
        let db = self.pool.clone().get().await.unwrap();
        let ids = queries::session::get_ids()
            .bind(&db)
            .all()
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        Ok(ids)
    }

    fn auto_handles_expiry(&self) -> bool {
        false
    }
}
