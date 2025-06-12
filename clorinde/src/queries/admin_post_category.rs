// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub id: T1,
    pub slug: T2,
    pub name: T3,
    pub meta_title: T4,
    pub meta_description: T5,
}
#[derive(Debug)]
pub struct UpdateParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub slug: T1,
    pub name: T2,
    pub meta_title: T3,
    pub meta_description: T4,
    pub id: T5,
}
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub slug: String,
    pub name: String,
}
pub struct ListBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub slug: &'a str,
    pub name: &'a str,
}
impl<'a> From<ListBorrowed<'a>> for List {
    fn from(
        ListBorrowed {
            id,
            created_at,
            slug,
            name,
        }: ListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            slug: slug.into(),
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub slug: String,
    pub name: String,
    pub meta_title: String,
    pub meta_description: String,
}
pub struct PageBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub slug: &'a str,
    pub name: &'a str,
    pub meta_title: &'a str,
    pub meta_description: &'a str,
}
impl<'a> From<PageBorrowed<'a>> for Page {
    fn from(
        PageBorrowed {
            id,
            created_at,
            slug,
            name,
            meta_title,
            meta_description,
        }: PageBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            slug: slug.into(),
            name: name.into(),
            meta_title: meta_title.into(),
            meta_description: meta_description.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
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
pub struct CrateTypesTimeTimestampQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor:
        fn(&tokio_postgres::Row) -> Result<crate::types::time::Timestamp, tokio_postgres::Error>,
    mapper: fn(crate::types::time::Timestamp) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(crate::types::time::Timestamp) -> R,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, R, N> {
        CrateTypesTimeTimestampQuery {
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
pub fn list() -> ListStmt {
    ListStmt(crate::client::async_::Stmt::new(
        "SELECT id, created_at, slug, name FROM \"PostCategory\"",
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
                    slug: row.try_get(2)?,
                    name: row.try_get(3)?,
                })
            },
            mapper: |it| List::from(it),
        }
    }
}
pub fn page() -> PageStmt {
    PageStmt(crate::client::async_::Stmt::new(
        "SELECT id, created_at, slug, name, meta_title, meta_description FROM \"PostCategory\" WHERE id = $1",
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
                    slug: row.try_get(2)?,
                    name: row.try_get(3)?,
                    meta_title: row.try_get(4)?,
                    meta_description: row.try_get(5)?,
                })
            },
            mapper: |it| Page::from(it),
        }
    }
}
pub fn create() -> CreateStmt {
    CreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"PostCategory\" (id, slug, name, meta_title, meta_description) VALUES ($1, $2, $3, $4, $5) RETURNING created_at",
    ))
}
pub struct CreateStmt(crate::client::async_::Stmt);
impl CreateStmt {
    pub fn bind<
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
        id: &'a T1,
        slug: &'a T2,
        name: &'a T3,
        meta_title: &'a T4,
        meta_description: &'a T5,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 5> {
        CrateTypesTimeTimestampQuery {
            client,
            params: [id, slug, name, meta_title, meta_description],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<
    'c,
    'a,
    's,
    C: GenericClient,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateParams<T1, T2, T3, T4, T5>,
        CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 5>,
        C,
    > for CreateStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a CreateParams<T1, T2, T3, T4, T5>,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 5> {
        self.bind(
            client,
            &params.id,
            &params.slug,
            &params.name,
            &params.meta_title,
            &params.meta_description,
        )
    }
}
pub fn update() -> UpdateStmt {
    UpdateStmt(crate::client::async_::Stmt::new(
        "UPDATE \"PostCategory\" SET slug = $1, name = $2, meta_title = $3, meta_description = $4 WHERE id = $5",
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
        slug: &'a T1,
        name: &'a T2,
        meta_title: &'a T3,
        meta_description: &'a T4,
        id: &'a T5,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[slug, name, meta_title, meta_description, id])
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
            &params.slug,
            &params.name,
            &params.meta_title,
            &params.meta_description,
            &params.id,
        ))
    }
}
pub fn delete() -> DeleteStmt {
    DeleteStmt(crate::client::async_::Stmt::new(
        "DELETE FROM \"PostCategory\" WHERE id = $1",
    ))
}
pub struct DeleteStmt(crate::client::async_::Stmt);
impl DeleteStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
