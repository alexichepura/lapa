// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub id: T1,
    pub alt: T2,
    pub ext: T3,
    pub product_id: T4,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReadByProduct {
    pub id: String,
    pub ext: String,
}
pub struct ReadByProductBorrowed<'a> {
    pub id: &'a str,
    pub ext: &'a str,
}
impl<'a> From<ReadByProductBorrowed<'a>> for ReadByProduct {
    fn from(ReadByProductBorrowed { id, ext }: ReadByProductBorrowed<'a>) -> Self {
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
pub struct ReadByProductQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<ReadByProductBorrowed, tokio_postgres::Error>,
    mapper: fn(ReadByProductBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReadByProductQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReadByProductBorrowed) -> R,
    ) -> ReadByProductQuery<'c, 'a, 's, C, R, N> {
        ReadByProductQuery {
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
pub fn create() -> CreateStmt {
    CreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"ProductImage\" (id, alt, ext, product_id) VALUES ($1, $2, $3, $4)",
    ))
}
pub struct CreateStmt(crate::client::async_::Stmt);
impl CreateStmt {
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
        alt: &'a T2,
        ext: &'a T3,
        product_id: &'a T4,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id, alt, ext, product_id]).await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a CreateParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.id,
            &params.alt,
            &params.ext,
            &params.product_id,
        ))
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
pub fn read_by_product() -> ReadByProductStmt {
    ReadByProductStmt(crate::client::async_::Stmt::new(
        "SELECT id, ext FROM \"ProductImage\" WHERE product_id = $1",
    ))
}
pub struct ReadByProductStmt(crate::client::async_::Stmt);
impl ReadByProductStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        product_id: &'a T1,
    ) -> ReadByProductQuery<'c, 'a, 's, C, ReadByProduct, 1> {
        ReadByProductQuery {
            client,
            params: [product_id],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<ReadByProductBorrowed, tokio_postgres::Error> {
                    Ok(ReadByProductBorrowed {
                        id: row.try_get(0)?,
                        ext: row.try_get(1)?,
                    })
                },
            mapper: |it| ReadByProduct::from(it),
        }
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
