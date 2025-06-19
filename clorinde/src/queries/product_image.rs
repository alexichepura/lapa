// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct UpdateAltParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub alt: T1,
    pub id: T2,
}
#[derive(Debug)]
pub struct UpdateOrderParams<T1: crate::StringSql> {
    pub order: i32,
    pub id: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectAllForConvert {
    pub id: String,
    pub ext: String,
}
pub struct SelectAllForConvertBorrowed<'a> {
    pub id: &'a str,
    pub ext: &'a str,
}
impl<'a> From<SelectAllForConvertBorrowed<'a>> for SelectAllForConvert {
    fn from(SelectAllForConvertBorrowed { id, ext }: SelectAllForConvertBorrowed<'a>) -> Self {
        Self {
            id: id.into(),
            ext: ext.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
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
pub struct SelectAllForConvertQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor:
        fn(&tokio_postgres::Row) -> Result<SelectAllForConvertBorrowed, tokio_postgres::Error>,
    mapper: fn(SelectAllForConvertBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectAllForConvertQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(SelectAllForConvertBorrowed) -> R,
    ) -> SelectAllForConvertQuery<'c, 'a, 's, C, R, N> {
        SelectAllForConvertQuery {
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
pub fn read_ext() -> ReadExtStmt {
    ReadExtStmt(crate::client::async_::Stmt::new(
        "SELECT ext FROM \"ProductImage\" WHERE id = $1",
    ))
}
pub struct ReadExtStmt(crate::client::async_::Stmt);
impl ReadExtStmt {
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
pub fn select_all_for_convert() -> SelectAllForConvertStmt {
    SelectAllForConvertStmt(crate::client::async_::Stmt::new(
        "SELECT id, ext FROM \"ProductImage\"",
    ))
}
pub struct SelectAllForConvertStmt(crate::client::async_::Stmt);
impl SelectAllForConvertStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> SelectAllForConvertQuery<'c, 'a, 's, C, SelectAllForConvert, 0> {
        SelectAllForConvertQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<SelectAllForConvertBorrowed, tokio_postgres::Error> {
                Ok(SelectAllForConvertBorrowed {
                    id: row.try_get(0)?,
                    ext: row.try_get(1)?,
                })
            },
            mapper: |it| SelectAllForConvert::from(it),
        }
    }
}
pub fn delete_many_by_id() -> DeleteManyByIdStmt {
    DeleteManyByIdStmt(crate::client::async_::Stmt::new(
        "DELETE FROM \"ProductImage\" WHERE id = ANY($1)",
    ))
}
pub struct DeleteManyByIdStmt(crate::client::async_::Stmt);
impl DeleteManyByIdStmt {
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = T1>,
    >(
        &'s mut self,
        client: &'c C,
        ids: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[ids]).await
    }
}
pub fn delete_by_id() -> DeleteByIdStmt {
    DeleteByIdStmt(crate::client::async_::Stmt::new(
        "DELETE FROM \"ProductImage\" WHERE id = $1",
    ))
}
pub struct DeleteByIdStmt(crate::client::async_::Stmt);
impl DeleteByIdStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn update_alt() -> UpdateAltStmt {
    UpdateAltStmt(crate::client::async_::Stmt::new(
        "UPDATE \"ProductImage\" SET alt = $1 WHERE id = $2",
    ))
}
pub struct UpdateAltStmt(crate::client::async_::Stmt);
impl UpdateAltStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        alt: &'a T1,
        id: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[alt, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateAltParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAltStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateAltParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.alt, &params.id))
    }
}
pub fn update_order() -> UpdateOrderStmt {
    UpdateOrderStmt(crate::client::async_::Stmt::new(
        "UPDATE \"ProductImage\" SET \"order\" = $1 WHERE id = $2",
    ))
}
pub struct UpdateOrderStmt(crate::client::async_::Stmt);
impl UpdateOrderStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        order: &'a i32,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[order, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateOrderParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateOrderStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateOrderParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.order, &params.id))
    }
}
pub fn set_hero() -> SetHeroStmt {
    SetHeroStmt(crate::client::async_::Stmt::new(
        "UPDATE \"ProductImage\" SET \"is_hero\" = true WHERE id = $1",
    ))
}
pub struct SetHeroStmt(crate::client::async_::Stmt);
impl SetHeroStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn unset_hero() -> UnsetHeroStmt {
    UnsetHeroStmt(crate::client::async_::Stmt::new(
        "UPDATE \"ProductImage\" SET \"is_hero\" = false WHERE id = $1",
    ))
}
pub struct UnsetHeroStmt(crate::client::async_::Stmt);
impl UnsetHeroStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn select_product_id() -> SelectProductIdStmt {
    SelectProductIdStmt(crate::client::async_::Stmt::new(
        "SELECT product_id FROM \"ProductImage\" WHERE id = $1",
    ))
}
pub struct SelectProductIdStmt(crate::client::async_::Stmt);
impl SelectProductIdStmt {
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
pub fn find_hero() -> FindHeroStmt {
    FindHeroStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"ProductImage\" WHERE product_id = $1 AND is_hero = true",
    ))
}
pub struct FindHeroStmt(crate::client::async_::Stmt);
impl FindHeroStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        product_id: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [product_id],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
