// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct SettingsCreateParams {
    pub hero_height: i32,
    pub hero_width: i32,
    pub thumb_height: i32,
    pub thumb_width: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub id: String,
    pub home_text: String,
    pub site_url: String,
    pub hero_height: i32,
    pub hero_width: i32,
    pub thumb_height: i32,
    pub thumb_width: i32,
}
pub struct SettingsBorrowed<'a> {
    pub id: &'a str,
    pub home_text: &'a str,
    pub site_url: &'a str,
    pub hero_height: i32,
    pub hero_width: i32,
    pub thumb_height: i32,
    pub thumb_width: i32,
}
impl<'a> From<SettingsBorrowed<'a>> for Settings {
    fn from(
        SettingsBorrowed {
            id,
            home_text,
            site_url,
            hero_height,
            hero_width,
            thumb_height,
            thumb_width,
        }: SettingsBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            home_text: home_text.into(),
            site_url: site_url.into(),
            hero_height,
            hero_width,
            thumb_height,
            thumb_width,
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
        "SELECT \"id\", \"home_text\", \"site_url\", \"hero_height\", \"hero_width\", \"thumb_height\", \"thumb_width\" FROM \"Settings\"",
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
                        hero_height: row.try_get(3)?,
                        hero_width: row.try_get(4)?,
                        thumb_height: row.try_get(5)?,
                        thumb_width: row.try_get(6)?,
                    })
                },
            mapper: |it| Settings::from(it),
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
        "INSERT INTO \"Settings\" (hero_height, hero_width, thumb_height, thumb_width) VALUES ($1, $2, $3, $4)",
    ))
}
pub struct SettingsCreateStmt(crate::client::async_::Stmt);
impl SettingsCreateStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        hero_height: &'a i32,
        hero_width: &'a i32,
        thumb_height: &'a i32,
        thumb_width: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[hero_height, hero_width, thumb_height, thumb_width])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        SettingsCreateParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for SettingsCreateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SettingsCreateParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.hero_height,
            &params.hero_width,
            &params.thumb_height,
            &params.thumb_width,
        ))
    }
}
