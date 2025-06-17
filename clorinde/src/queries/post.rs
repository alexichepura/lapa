// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct PageParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub category_slug: T1,
    pub slug: T2,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: crate::types::time::Timestamp,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub content_id: String,
    pub content_json: String,
}
pub struct PageBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: crate::types::time::Timestamp,
    pub slug: &'a str,
    pub meta_title: &'a str,
    pub meta_description: &'a str,
    pub content_id: &'a str,
    pub content_json: &'a str,
}
impl<'a> From<PageBorrowed<'a>> for Page {
    fn from(
        PageBorrowed {
            id,
            created_at,
            publish_at,
            slug,
            meta_title,
            meta_description,
            content_id,
            content_json,
        }: PageBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            publish_at,
            slug: slug.into(),
            meta_title: meta_title.into(),
            meta_description: meta_description.into(),
            content_id: content_id.into(),
            content_json: content_json.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct PageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<PageBorrowed, tokio_postgres::Error>,
    mapper: fn(PageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(PageBorrowed) -> R) -> PageQuery<'c, 'a, 's, C, R, N> {
        PageQuery {
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
pub fn page() -> PageStmt {
    PageStmt(crate::client::async_::Stmt::new(
        "SELECT \"Post\".id, \"Post\".created_at, \"Post\".publish_at, \"Post\".slug, \"Post\".meta_title, \"Post\".meta_description, \"Content\".id AS content_id, \"Content\".json AS content_json FROM \"Post\" INNER JOIN \"Content\" ON \"Content\".id = \"Post\".content_id INNER JOIN \"PostCategory\" ON \"PostCategory\".id = \"Post\".category_id WHERE \"PostCategory\".slug = $1 AND \"Post\".slug = $2 AND \"Post\".publish_at < NOW()",
    ))
}
pub struct PageStmt(crate::client::async_::Stmt);
impl PageStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        category_slug: &'a T1,
        slug: &'a T2,
    ) -> PageQuery<'c, 'a, 's, C, Page, 2> {
        PageQuery {
            client,
            params: [category_slug, slug],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<PageBorrowed, tokio_postgres::Error> {
                Ok(PageBorrowed {
                    id: row.try_get(0)?,
                    created_at: row.try_get(1)?,
                    publish_at: row.try_get(2)?,
                    slug: row.try_get(3)?,
                    meta_title: row.try_get(4)?,
                    meta_description: row.try_get(5)?,
                    content_id: row.try_get(6)?,
                    content_json: row.try_get(7)?,
                })
            },
            mapper: |it| Page::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        PageParams<T1, T2>,
        PageQuery<'c, 'a, 's, C, Page, 2>,
        C,
    > for PageStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a PageParams<T1, T2>,
    ) -> PageQuery<'c, 'a, 's, C, Page, 2> {
        self.bind(client, &params.category_slug, &params.slug)
    }
}
