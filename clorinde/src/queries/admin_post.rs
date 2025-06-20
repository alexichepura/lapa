// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
> {
    pub id: T1,
    pub slug: T2,
    pub meta_title: T3,
    pub meta_description: T4,
    pub h1: T5,
    pub content_id: T6,
    pub category_id: T7,
}
#[derive(Debug)]
pub struct UpdateParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub slug: T1,
    pub meta_title: T2,
    pub meta_description: T3,
    pub h1: T4,
    pub id: T5,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub h1: String,
    pub content_id: String,
    pub content_json: String,
}
pub struct PageBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
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
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub h1: String,
}
pub struct ListBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub h1: &'a str,
}
impl<'a> From<ListBorrowed<'a>> for List {
    fn from(
        ListBorrowed {
            id,
            created_at,
            publish_at,
            h1,
        }: ListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            publish_at,
            h1: h1.into(),
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
pub struct ListQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
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
pub fn create() -> CreateStmt {
    CreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"Post\" (id, slug, meta_title, meta_description, h1, content_id, category_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
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
        T5: crate::StringSql,
        T6: crate::StringSql,
        T7: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
        slug: &'a T2,
        meta_title: &'a T3,
        meta_description: &'a T4,
        h1: &'a T5,
        content_id: &'a T6,
        category_id: &'a T7,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(
                stmt,
                &[
                    id,
                    slug,
                    meta_title,
                    meta_description,
                    h1,
                    content_id,
                    category_id,
                ],
            )
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateParams<T1, T2, T3, T4, T5, T6, T7>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a CreateParams<T1, T2, T3, T4, T5, T6, T7>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.id,
            &params.slug,
            &params.meta_title,
            &params.meta_description,
            &params.h1,
            &params.content_id,
            &params.category_id,
        ))
    }
}
pub fn page() -> PageStmt {
    PageStmt(crate::client::async_::Stmt::new(
        "SELECT \"Post\".id, \"Post\".created_at, \"Post\".publish_at, \"Post\".slug, \"Post\".meta_title, \"Post\".meta_description, \"Post\".h1, \"Content\".id AS content_id, \"Content\".json AS content_json FROM \"Post\" INNER JOIN \"Content\" ON \"Content\".id = \"Post\".content_id WHERE \"Post\".id = $1",
    ))
}
pub struct PageStmt(crate::client::async_::Stmt);
impl PageStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> PageQuery<'c, 'a, 's, C, Page, 1> {
        PageQuery {
            client,
            params: [id],
            stmt: &mut self.0,
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
pub fn update() -> UpdateStmt {
    UpdateStmt(crate::client::async_::Stmt::new(
        "UPDATE \"Post\" SET publish_at = $1, slug = $2, meta_title = $3, meta_description = $4, h1 = $5 WHERE id = $6",
    ))
}
pub struct UpdateStmt(crate::client::async_::Stmt);
impl UpdateStmt {
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        publish_at: &'a Option<crate::types::time::Timestamp>,
        slug: &'a T1,
        meta_title: &'a T2,
        meta_description: &'a T3,
        h1: &'a T4,
        id: &'a T5,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(
                stmt,
                &[publish_at, slug, meta_title, meta_description, h1, id],
            )
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateParams<T1, T2, T3, T4, T5>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateParams<T1, T2, T3, T4, T5>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.publish_at,
            &params.slug,
            &params.meta_title,
            &params.meta_description,
            &params.h1,
            &params.id,
        ))
    }
}
pub fn list() -> ListStmt {
    ListStmt(crate::client::async_::Stmt::new(
        "SELECT id, created_at, publish_at, h1 FROM \"Post\"",
    ))
}
pub struct ListStmt(crate::client::async_::Stmt);
impl ListStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> ListQuery<'c, 'a, 's, C, List, 0> {
        ListQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<ListBorrowed, tokio_postgres::Error> {
                Ok(ListBorrowed {
                    id: row.try_get(0)?,
                    created_at: row.try_get(1)?,
                    publish_at: row.try_get(2)?,
                    h1: row.try_get(3)?,
                })
            },
            mapper: |it| List::from(it),
        }
    }
}
pub fn read_content_id() -> ReadContentIdStmt {
    ReadContentIdStmt(crate::client::async_::Stmt::new(
        "SELECT content_id FROM \"Post\" WHERE id = $1",
    ))
}
pub struct ReadContentIdStmt(crate::client::async_::Stmt);
impl ReadContentIdStmt {
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
pub fn by_slug() -> BySlugStmt {
    BySlugStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"Post\" WHERE slug = $1",
    ))
}
pub struct BySlugStmt(crate::client::async_::Stmt);
impl BySlugStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        slug: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [slug],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
