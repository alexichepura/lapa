// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct PageParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub category_slug: T1,
    pub slug: T2,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub id: String,
    pub created_at: chrono::NaiveDateTime,
    pub publish_at: chrono::NaiveDateTime,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub h1: String,
    pub content_id: String,
    pub content_json: String,
}
pub struct PageBorrowed<'a> {
    pub id: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub publish_at: chrono::NaiveDateTime,
    pub slug: &'a str,
    pub meta_title: &'a str,
    pub meta_description: &'a str,
    pub h1: &'a str,
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
            h1,
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
            h1: h1.into(),
            content_id: content_id.into(),
            content_json: content_json.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub id: String,
    pub publish_at: chrono::NaiveDateTime,
    pub slug: String,
    pub h1: String,
    pub category_slug: String,
}
pub struct ListBorrowed<'a> {
    pub id: &'a str,
    pub publish_at: chrono::NaiveDateTime,
    pub slug: &'a str,
    pub h1: &'a str,
    pub category_slug: &'a str,
}
impl<'a> From<ListBorrowed<'a>> for List {
    fn from(
        ListBorrowed {
            id,
            publish_at,
            slug,
            h1,
            category_slug,
        }: ListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            publish_at,
            slug: slug.into(),
            h1: h1.into(),
            category_slug: category_slug.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct PageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
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
pub struct ListQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ListBorrowed, tokio_postgres::Error>,
    mapper: fn(ListBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ListQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ListBorrowed) -> R) -> ListQuery<'c, 'a, 's, C, R, N> {
        ListQuery {
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
pub struct PageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn page() -> PageStmt {
    PageStmt(
        "SELECT \"Post\".id, \"Post\".created_at, \"Post\".publish_at, \"Post\".slug, \"Post\".meta_title, \"Post\".meta_description, \"Post\".h1, \"Content\".id AS content_id, \"Content\".json AS content_json FROM \"Post\" INNER JOIN \"Content\" ON \"Content\".id = \"Post\".content_id INNER JOIN \"PostCategory\" ON \"PostCategory\".id = \"Post\".category_id WHERE \"PostCategory\".slug = $1 AND \"Post\".slug = $2 AND \"Post\".publish_at < NOW()",
        None,
    )
}
impl PageStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        category_slug: &'a T1,
        slug: &'a T2,
    ) -> PageQuery<'c, 'a, 's, C, Page, 2> {
        PageQuery {
            client,
            params: [category_slug, slug],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<PageBorrowed, tokio_postgres::Error> {
                Ok(PageBorrowed {
                    id: row.try_get(0)?,
                    created_at: row.try_get(1)?,
                    publish_at: row.try_get(2)?,
                    slug: row.try_get(3)?,
                    meta_title: row.try_get(4)?,
                    meta_description: row.try_get(5)?,
                    h1: row.try_get(6)?,
                    content_id: row.try_get(7)?,
                    content_json: row.try_get(8)?,
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
        &'s self,
        client: &'c C,
        params: &'a PageParams<T1, T2>,
    ) -> PageQuery<'c, 'a, 's, C, Page, 2> {
        self.bind(client, &params.category_slug, &params.slug)
    }
}
pub struct ListStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list() -> ListStmt {
    ListStmt(
        "SELECT \"Post\".id, \"Post\".publish_at, \"Post\".slug, \"Post\".h1, \"PostCategory\".slug AS category_slug FROM \"Post\" INNER JOIN \"PostCategory\" ON \"PostCategory\".id = \"Post\".category_id WHERE \"Post\".publish_at < NOW()",
        None,
    )
}
impl ListStmt {
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
    ) -> ListQuery<'c, 'a, 's, C, List, 0> {
        ListQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<ListBorrowed, tokio_postgres::Error> {
                Ok(ListBorrowed {
                    id: row.try_get(0)?,
                    publish_at: row.try_get(1)?,
                    slug: row.try_get(2)?,
                    h1: row.try_get(3)?,
                    category_slug: row.try_get(4)?,
                })
            },
            mapper: |it| List::from(it),
        }
    }
}
