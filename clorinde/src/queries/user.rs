// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct UserCreateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub id: T1,
    pub username: T2,
    pub password: T3,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserFindByUsername {
    pub id: String,
    pub password: String,
}
pub struct UserFindByUsernameBorrowed<'a> {
    pub id: &'a str,
    pub password: &'a str,
}
impl<'a> From<UserFindByUsernameBorrowed<'a>> for UserFindByUsername {
    fn from(UserFindByUsernameBorrowed { id, password }: UserFindByUsernameBorrowed<'a>) -> Self {
        Self {
            id: id.into(),
            password: password.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UserFindByUsernameQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor:
        fn(&tokio_postgres::Row) -> Result<UserFindByUsernameBorrowed, tokio_postgres::Error>,
    mapper: fn(UserFindByUsernameBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserFindByUsernameQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UserFindByUsernameBorrowed) -> R,
    ) -> UserFindByUsernameQuery<'c, 'a, 's, C, R, N> {
        UserFindByUsernameQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
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
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
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
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub fn user_create() -> UserCreateStmt {
    UserCreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"User\" (id, username, password) VALUES ($1, $2, $3)",
    ))
}
pub struct UserCreateStmt(crate::client::async_::Stmt);
impl UserCreateStmt {
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
        username: &'a T2,
        password: &'a T3,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id, username, password]).await
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
        UserCreateParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UserCreateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UserCreateParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.username, &params.password))
    }
}
pub fn user_find_by_username() -> UserFindByUsernameStmt {
    UserFindByUsernameStmt(crate::client::async_::Stmt::new(
        "SELECT id, password FROM \"User\" WHERE username = $1",
    ))
}
pub struct UserFindByUsernameStmt(crate::client::async_::Stmt);
impl UserFindByUsernameStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        username: &'a T1,
    ) -> UserFindByUsernameQuery<'c, 'a, 's, C, UserFindByUsername, 1> {
        UserFindByUsernameQuery {
            client,
            params: [username],
            stmt: &mut self.0,
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<UserFindByUsernameBorrowed, tokio_postgres::Error> {
                Ok(UserFindByUsernameBorrowed {
                    id: row.try_get(0)?,
                    password: row.try_get(1)?,
                })
            },
            mapper: |it| UserFindByUsername::from(it),
        }
    }
}
pub fn user_find_by_id() -> UserFindByIdStmt {
    UserFindByIdStmt(crate::client::async_::Stmt::new(
        "SELECT username FROM \"User\" WHERE id = $1",
    ))
}
pub struct UserFindByIdStmt(crate::client::async_::Stmt);
impl UserFindByIdStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
