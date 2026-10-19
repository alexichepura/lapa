// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct StoreParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub id: T1,
    pub session: T2,
    pub expires: chrono::NaiveDateTime,
    pub user_id: Option<T3>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Load {
    pub session: String,
    pub expires: chrono::NaiveDateTime,
}
pub struct LoadBorrowed<'a> {
    pub session: &'a str,
    pub expires: chrono::NaiveDateTime,
}
impl<'a> From<LoadBorrowed<'a>> for Load {
    fn from(LoadBorrowed { session, expires }: LoadBorrowed<'a>) -> Self {
        Self {
            session: session.into(),
            expires,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct LoadQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<LoadBorrowed, tokio_postgres::Error>,
    mapper: fn(LoadBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> LoadQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(LoadBorrowed) -> R) -> LoadQuery<'c, 'a, 's, C, R, N> {
        LoadQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct StoreStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn store() -> StoreStmt {
    StoreStmt(
        "INSERT INTO \"Session\" (id, session, expires, user_id) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET session = EXCLUDED.session, expires = EXCLUDED.expires, user_id = EXCLUDED.user_id",
        None,
    )
}
impl StoreStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        id: &'a T1,
        session: &'a T2,
        expires: &'a chrono::NaiveDateTime,
        user_id: &'a Option<T3>,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[id, session, expires, user_id])
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        StoreParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for StoreStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a StoreParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.id,
            &params.session,
            &params.expires,
            &params.user_id,
        ))
    }
}
pub struct LoadStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn load() -> LoadStmt {
    LoadStmt(
        "SELECT session, expires FROM \"Session\" WHERE id = $1 AND expires > NOW()",
        None,
    )
}
impl LoadStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        id: &'a T1,
    ) -> LoadQuery<'c, 'a, 's, C, Load, 1> {
        LoadQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<LoadBorrowed, tokio_postgres::Error> {
                Ok(LoadBorrowed {
                    session: row.try_get(0)?,
                    expires: row.try_get(1)?,
                })
            },
            mapper: |it| Load::from(it),
        }
    }
}
pub struct DeleteOneByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_one_by_id() -> DeleteOneByIdStmt {
    DeleteOneByIdStmt("DELETE FROM \"Session\" WHERE id = $1", None)
}
impl DeleteOneByIdStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id]).await
    }
}
pub struct DeleteExpiredStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_expired() -> DeleteExpiredStmt {
    DeleteExpiredStmt(
        "DELETE FROM \"Session\" WHERE expires IS NOT NULL AND expires < NOW()",
        None,
    )
}
impl DeleteExpiredStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[]).await
    }
}
pub struct DeleteByUsernameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_by_username() -> DeleteByUsernameStmt {
    DeleteByUsernameStmt(
        "DELETE FROM \"Session\" WHERE user_id = (SELECT id FROM \"User\" WHERE username = $1)",
        None,
    )
}
impl DeleteByUsernameStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        username: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[username]).await
    }
}
