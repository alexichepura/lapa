// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct UserCreateParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub username: T1,
    pub password: T2,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub fn user_create() -> UserCreateStmt {
    UserCreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"User\" (username, password) VALUES ($1, $2)",
    ))
}
pub struct UserCreateStmt(crate::client::async_::Stmt);
impl UserCreateStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        username: &'a T1,
        password: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[username, password]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UserCreateParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UserCreateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UserCreateParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.username, &params.password))
    }
}
