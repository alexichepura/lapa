// This file was generated with `clorinde`. Do not modify.

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
pub fn select_all_for_convert() -> SelectAllForConvertStmt {
    SelectAllForConvertStmt(crate::client::async_::Stmt::new(
        "SELECT id, ext FROM \"Image\"",
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
        "DELETE FROM \"Image\" WHERE id = ANY($1)",
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
