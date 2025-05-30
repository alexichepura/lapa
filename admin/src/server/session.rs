use async_trait::async_trait;
use axum_session::{
    DatabaseError, DatabasePool, SessionConfig, SessionLayer, SessionStore,
};
use clorinde::deadpool_postgres::Pool;
use std::vec;

pub async fn session_layer(prisma: &Pool) -> SessionLayer<SessionPool> {
    let config = SessionConfig::default()
        .with_table_name("Session")
        .with_session_name("session");

    let store = SessionStore::<SessionPool>::new(Some(prisma.clone().into()), config)
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
        let result = self
            .pool
            .session()
            .find_many(vec![session::expires::lt(Utc::now().timestamp() as i32)])
            .select(db::session::select!({ id }))
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;

        let ids: Vec<String> = result.iter().map(|r| r.id.clone()).collect();

        let _result = self
            .pool
            .session()
            // .delete_many(vec![session::expires::lt(Utc::now().timestamp() as i32)])
            .delete_many(vec![session::id::in_vec(ids.clone())])
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(ids)
    }

    async fn count(&self, _table_name: &str) -> Result<i64, DatabaseError> {
        let count = self
            .pool
            .session()
            .count(vec![])
            .exec()
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
        self.pool
            .session()
            .upsert(
                session::id::equals(id.to_string()),
                session::create(
                    id.to_string(),
                    session.to_string(),
                    vec![session::expires::set(Some(expires as i32))],
                ),
                vec![session::expires::set(Some(expires as i32))],
            )
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericCreateError(e.to_string()))?;
        Ok(())
    }

    async fn load(&self, id: &str, _table_name: &str) -> Result<Option<String>, DatabaseError> {
        let result = self
            .pool
            .session()
            .find_first(vec![
                session::id::equals(id.to_string()),
                prisma_client_rust::or!(
                    session::expires::equals(None),
                    session::expires::gt(Utc::now().timestamp() as i32)
                ),
            ])
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;

        Ok(match result {
            Some(result) => Some(result.session),
            None => None,
        })
    }

    async fn delete_one_by_id(&self, id: &str, _table_name: &str) -> Result<(), DatabaseError> {
        self.pool
            .session()
            .delete(session::id::equals(id.to_string()))
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn exists(&self, id: &str, _table_name: &str) -> Result<bool, DatabaseError> {
        let result = self
            .pool
            .session()
            .count(vec![
                session::id::equals(id.to_string()),
                prisma_client_rust::or!(
                    session::expires::equals(None),
                    session::expires::gt(Utc::now().timestamp() as i32)
                ),
            ])
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        let exists = result > 0;
        Ok(exists)
    }

    async fn delete_all(&self, _table_name: &str) -> Result<(), DatabaseError> {
        tracing::debug!("delete_all");
        self.pool
            .session()
            .delete_many(vec![])
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn get_ids(&self, _table_name: &str) -> Result<Vec<String>, DatabaseError> {
        let result = self
            .pool
            .session()
            .find_many(vec![prisma_client_rust::or!(
                session::expires::equals(None),
                session::expires::gt(Utc::now().timestamp() as i32)
            )])
            .select(db::session::select!({ id }))
            .exec()
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;

        let result: Vec<String> = result.iter().map(|data| data.id.clone()).collect();
        Ok(result)
    }

    fn auto_handles_expiry(&self) -> bool {
        false
    }
}
