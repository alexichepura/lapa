// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct UserCreateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub id: T1,
    pub username: T2,
    pub password: T3,
}
#[derive(Debug)]
pub struct UserSetActiveParams<T1: crate::StringSql> {
    pub active: bool,
    pub username: T1,
}
#[derive(Debug)]
pub struct UserSetRoleParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub role: T1,
    pub username: T2,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserGetAuthByUsername {
    pub id: String,
    pub password: String,
    pub deactivated_at: Option<chrono::NaiveDateTime>,
    pub role: String,
}
pub struct UserGetAuthByUsernameBorrowed<'a> {
    pub id: &'a str,
    pub password: &'a str,
    pub deactivated_at: Option<chrono::NaiveDateTime>,
    pub role: &'a str,
}
impl<'a> From<UserGetAuthByUsernameBorrowed<'a>> for UserGetAuthByUsername {
    fn from(
        UserGetAuthByUsernameBorrowed {
            id,
            password,
            deactivated_at,
            role,
        }: UserGetAuthByUsernameBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            password: password.into(),
            deactivated_at,
            role: role.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserFindById {
    pub username: String,
    pub password: String,
    pub deactivated_at: Option<chrono::NaiveDateTime>,
    pub role: String,
}
pub struct UserFindByIdBorrowed<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub deactivated_at: Option<chrono::NaiveDateTime>,
    pub role: &'a str,
}
impl<'a> From<UserFindByIdBorrowed<'a>> for UserFindById {
    fn from(
        UserFindByIdBorrowed {
            username,
            password,
            deactivated_at,
            role,
        }: UserFindByIdBorrowed<'a>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            deactivated_at,
            role: role.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UserGetAuthByUsernameQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<UserGetAuthByUsernameBorrowed, tokio_postgres::Error>,
    mapper: fn(UserGetAuthByUsernameBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserGetAuthByUsernameQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UserGetAuthByUsernameBorrowed) -> R,
    ) -> UserGetAuthByUsernameQuery<'c, 'a, 's, C, R, N> {
        UserGetAuthByUsernameQuery {
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
pub struct UserFindByIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UserFindByIdBorrowed, tokio_postgres::Error>,
    mapper: fn(UserFindByIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserFindByIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UserFindByIdBorrowed) -> R,
    ) -> UserFindByIdQuery<'c, 'a, 's, C, R, N> {
        UserFindByIdQuery {
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
pub struct UserCreateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn user_create() -> UserCreateStmt {
    UserCreateStmt(
        "INSERT INTO \"User\" (id, username, password) VALUES ($1, $2, $3)",
        None,
    )
}
impl UserCreateStmt {
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
        username: &'a T2,
        password: &'a T3,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id, username, password]).await
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
        &'a self,
        client: &'a C,
        params: &'a UserCreateParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.username, &params.password))
    }
}
pub struct UserGetAuthByUsernameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn user_get_auth_by_username() -> UserGetAuthByUsernameStmt {
    UserGetAuthByUsernameStmt(
        "SELECT id, password, deactivated_at, role FROM \"User\" WHERE username = $1",
        None,
    )
}
impl UserGetAuthByUsernameStmt {
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
        username: &'a T1,
    ) -> UserGetAuthByUsernameQuery<'c, 'a, 's, C, UserGetAuthByUsername, 1> {
        UserGetAuthByUsernameQuery {
            client,
            params: [username],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<UserGetAuthByUsernameBorrowed, tokio_postgres::Error> {
                Ok(UserGetAuthByUsernameBorrowed {
                    id: row.try_get(0)?,
                    password: row.try_get(1)?,
                    deactivated_at: row.try_get(2)?,
                    role: row.try_get(3)?,
                })
            },
            mapper: |it| UserGetAuthByUsername::from(it),
        }
    }
}
pub struct UserFindByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn user_find_by_id() -> UserFindByIdStmt {
    UserFindByIdStmt(
        "SELECT username, password, deactivated_at, role FROM \"User\" WHERE id = $1",
        None,
    )
}
impl UserFindByIdStmt {
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
    ) -> UserFindByIdQuery<'c, 'a, 's, C, UserFindById, 1> {
        UserFindByIdQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UserFindByIdBorrowed, tokio_postgres::Error> {
                    Ok(UserFindByIdBorrowed {
                        username: row.try_get(0)?,
                        password: row.try_get(1)?,
                        deactivated_at: row.try_get(2)?,
                        role: row.try_get(3)?,
                    })
                },
            mapper: |it| UserFindById::from(it),
        }
    }
}
pub struct UserSetActiveStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn user_set_active() -> UserSetActiveStmt {
    UserSetActiveStmt(
        "UPDATE \"User\" SET deactivated_at = CASE WHEN $1 THEN NULL ELSE COALESCE(deactivated_at, now()) END WHERE username = $2",
        None,
    )
}
impl UserSetActiveStmt {
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
        active: &'a bool,
        username: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[active, username]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UserSetActiveParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UserSetActiveStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UserSetActiveParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.active, &params.username))
    }
}
pub struct UserSetRoleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn user_set_role() -> UserSetRoleStmt {
    UserSetRoleStmt("UPDATE \"User\" SET role = $1 WHERE username = $2", None)
}
impl UserSetRoleStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        role: &'a T1,
        username: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[role, username]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UserSetRoleParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UserSetRoleStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UserSetRoleParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.role, &params.username))
    }
}
