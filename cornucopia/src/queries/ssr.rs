// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub id: T1,
    pub path: T2,
    pub user_agent: Option<T3>,
}
#[derive(Clone, Copy, Debug)]
pub struct StatsAllParams {
    pub limit: i64,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct StatsSinceParams {
    pub since: chrono::NaiveDateTime,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StatsAll {
    pub path: String,
    pub n: i64,
}
pub struct StatsAllBorrowed<'a> {
    pub path: &'a str,
    pub n: i64,
}
impl<'a> From<StatsAllBorrowed<'a>> for StatsAll {
    fn from(StatsAllBorrowed { path, n }: StatsAllBorrowed<'a>) -> Self {
        Self {
            path: path.into(),
            n,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct StatsSince {
    pub path: String,
    pub n: i64,
}
pub struct StatsSinceBorrowed<'a> {
    pub path: &'a str,
    pub n: i64,
}
impl<'a> From<StatsSinceBorrowed<'a>> for StatsSince {
    fn from(StatsSinceBorrowed { path, n }: StatsSinceBorrowed<'a>) -> Self {
        Self {
            path: path.into(),
            n,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct StatsAllQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<StatsAllBorrowed, tokio_postgres::Error>,
    mapper: fn(StatsAllBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StatsAllQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(StatsAllBorrowed) -> R) -> StatsAllQuery<'c, 'a, 's, C, R, N> {
        StatsAllQuery {
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
pub struct StatsSinceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<StatsSinceBorrowed, tokio_postgres::Error>,
    mapper: fn(StatsSinceBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StatsSinceQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(StatsSinceBorrowed) -> R,
    ) -> StatsSinceQuery<'c, 'a, 's, C, R, N> {
        StatsSinceQuery {
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
pub struct CreateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create() -> CreateStmt {
    CreateStmt(
        "INSERT INTO \"Ssr\" (id, path, user_agent) VALUES ($1, $2, $3)",
        None,
    )
}
impl CreateStmt {
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
        path: &'a T2,
        user_agent: &'a Option<T3>,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id, path, user_agent]).await
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
        CreateParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a CreateParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.path, &params.user_agent))
    }
}
pub struct StatsAllStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn stats_all() -> StatsAllStmt {
    StatsAllStmt(
        "SELECT path, count(*) AS n FROM \"Ssr\" GROUP BY path ORDER BY n DESC, path LIMIT $1 OFFSET $2",
        None,
    )
}
impl StatsAllStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        limit: &'a i64,
        offset: &'a i64,
    ) -> StatsAllQuery<'c, 'a, 's, C, StatsAll, 2> {
        StatsAllQuery {
            client,
            params: [limit, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<StatsAllBorrowed, tokio_postgres::Error> {
                    Ok(StatsAllBorrowed {
                        path: row.try_get(0)?,
                        n: row.try_get(1)?,
                    })
                },
            mapper: |it| StatsAll::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        StatsAllParams,
        StatsAllQuery<'c, 'a, 's, C, StatsAll, 2>,
        C,
    > for StatsAllStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a StatsAllParams,
    ) -> StatsAllQuery<'c, 'a, 's, C, StatsAll, 2> {
        self.bind(client, &params.limit, &params.offset)
    }
}
pub struct StatsSinceStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn stats_since() -> StatsSinceStmt {
    StatsSinceStmt(
        "SELECT path, count(*) AS n FROM \"Ssr\" WHERE created_at > $1 GROUP BY path ORDER BY n DESC, path LIMIT $2 OFFSET $3",
        None,
    )
}
impl StatsSinceStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        since: &'a chrono::NaiveDateTime,
        limit: &'a i64,
        offset: &'a i64,
    ) -> StatsSinceQuery<'c, 'a, 's, C, StatsSince, 3> {
        StatsSinceQuery {
            client,
            params: [since, limit, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<StatsSinceBorrowed, tokio_postgres::Error> {
                    Ok(StatsSinceBorrowed {
                        path: row.try_get(0)?,
                        n: row.try_get(1)?,
                    })
                },
            mapper: |it| StatsSince::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        StatsSinceParams,
        StatsSinceQuery<'c, 'a, 's, C, StatsSince, 3>,
        C,
    > for StatsSinceStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a StatsSinceParams,
    ) -> StatsSinceQuery<'c, 'a, 's, C, StatsSince, 3> {
        self.bind(client, &params.since, &params.limit, &params.offset)
    }
}
