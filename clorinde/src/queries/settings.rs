// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct SettingsUpdateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub robots_txt: T1,
    pub site_url: T2,
    pub id: T3,
}
#[derive(Debug)]
pub struct SettingsUpdateHomeParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub home_text: T1,
    pub id: T2,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub id: String,
    pub home_text: String,
    pub site_url: String,
}
pub struct SettingsBorrowed<'a> {
    pub id: &'a str,
    pub home_text: &'a str,
    pub site_url: &'a str,
}
impl<'a> From<SettingsBorrowed<'a>> for Settings {
    fn from(
        SettingsBorrowed {
            id,
            home_text,
            site_url,
        }: SettingsBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            home_text: home_text.into(),
            site_url: site_url.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsPage {
    pub id: String,
    pub robots_txt: String,
    pub home_text: String,
    pub site_url: String,
}
pub struct SettingsPageBorrowed<'a> {
    pub id: &'a str,
    pub robots_txt: &'a str,
    pub home_text: &'a str,
    pub site_url: &'a str,
}
impl<'a> From<SettingsPageBorrowed<'a>> for SettingsPage {
    fn from(
        SettingsPageBorrowed {
            id,
            robots_txt,
            home_text,
            site_url,
        }: SettingsPageBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            robots_txt: robots_txt.into(),
            home_text: home_text.into(),
            site_url: site_url.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct SettingsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<SettingsBorrowed, tokio_postgres::Error>,
    mapper: fn(SettingsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SettingsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(SettingsBorrowed) -> R) -> SettingsQuery<'c, 'a, 's, C, R, N> {
        SettingsQuery {
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
pub struct SettingsPageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<SettingsPageBorrowed, tokio_postgres::Error>,
    mapper: fn(SettingsPageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SettingsPageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(SettingsPageBorrowed) -> R,
    ) -> SettingsPageQuery<'c, 'a, 's, C, R, N> {
        SettingsPageQuery {
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
pub fn settings() -> SettingsStmt {
    SettingsStmt(crate::client::async_::Stmt::new(
        "SELECT \"id\", \"home_text\", \"site_url\" FROM \"Settings\"",
    ))
}
pub struct SettingsStmt(crate::client::async_::Stmt);
impl SettingsStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> SettingsQuery<'c, 'a, 's, C, Settings, 0> {
        SettingsQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<SettingsBorrowed, tokio_postgres::Error> {
                    Ok(SettingsBorrowed {
                        id: row.try_get(0)?,
                        home_text: row.try_get(1)?,
                        site_url: row.try_get(2)?,
                    })
                },
            mapper: |it| Settings::from(it),
        }
    }
}
pub fn settings_page() -> SettingsPageStmt {
    SettingsPageStmt(crate::client::async_::Stmt::new(
        "SELECT \"id\", \"robots_txt\", \"home_text\", \"site_url\" FROM \"Settings\"",
    ))
}
pub struct SettingsPageStmt(crate::client::async_::Stmt);
impl SettingsPageStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> SettingsPageQuery<'c, 'a, 's, C, SettingsPage, 0> {
        SettingsPageQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<SettingsPageBorrowed, tokio_postgres::Error> {
                    Ok(SettingsPageBorrowed {
                        id: row.try_get(0)?,
                        robots_txt: row.try_get(1)?,
                        home_text: row.try_get(2)?,
                        site_url: row.try_get(3)?,
                    })
                },
            mapper: |it| SettingsPage::from(it),
        }
    }
}
pub fn settings_robots() -> SettingsRobotsStmt {
    SettingsRobotsStmt(crate::client::async_::Stmt::new(
        "SELECT \"robots_txt\" FROM \"Settings\"",
    ))
}
pub struct SettingsRobotsStmt(crate::client::async_::Stmt);
impl SettingsRobotsStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> StringQuery<'c, 'a, 's, C, String, 0> {
        StringQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub fn settings_home() -> SettingsHomeStmt {
    SettingsHomeStmt(crate::client::async_::Stmt::new(
        "SELECT \"home_text\" FROM \"Settings\"",
    ))
}
pub struct SettingsHomeStmt(crate::client::async_::Stmt);
impl SettingsHomeStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> StringQuery<'c, 'a, 's, C, String, 0> {
        StringQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub fn settings_create() -> SettingsCreateStmt {
    SettingsCreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"Settings\" (id) VALUES ($1)",
    ))
}
pub struct SettingsCreateStmt(crate::client::async_::Stmt);
impl SettingsCreateStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn settings_update() -> SettingsUpdateStmt {
    SettingsUpdateStmt(crate::client::async_::Stmt::new(
        "UPDATE \"Settings\" SET robots_txt = $1, site_url = $2 WHERE id = $3",
    ))
}
pub struct SettingsUpdateStmt(crate::client::async_::Stmt);
impl SettingsUpdateStmt {
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
        robots_txt: &'a T1,
        site_url: &'a T2,
        id: &'a T3,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[robots_txt, site_url, id]).await
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
        SettingsUpdateParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for SettingsUpdateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SettingsUpdateParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.robots_txt, &params.site_url, &params.id))
    }
}
pub fn settings_update_home() -> SettingsUpdateHomeStmt {
    SettingsUpdateHomeStmt(crate::client::async_::Stmt::new(
        "UPDATE \"Settings\" SET home_text = $1 WHERE id = $2",
    ))
}
pub struct SettingsUpdateHomeStmt(crate::client::async_::Stmt);
impl SettingsUpdateHomeStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        home_text: &'a T1,
        id: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[home_text, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        SettingsUpdateHomeParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for SettingsUpdateHomeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SettingsUpdateHomeParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.home_text, &params.id))
    }
}
