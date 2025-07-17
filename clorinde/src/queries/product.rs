// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub id: String,
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
    pub image_id: Option<String>,
    pub alt: Option<String>,
}
pub struct ListBorrowed<'a> {
    pub id: &'a str,
    pub publish_at: chrono::NaiveDateTime,
    pub slug: &'a str,
    pub h1: &'a str,
    pub image_id: Option<&'a str>,
    pub alt: Option<&'a str>,
}
impl<'a> From<ListBorrowed<'a>> for List {
    fn from(
        ListBorrowed {
            id,
            publish_at,
            slug,
            h1,
            image_id,
            alt,
        }: ListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            publish_at,
            slug: slug.into(),
            h1: h1.into(),
            image_id: image_id.map(|v| v.into()),
            alt: alt.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Images {
    pub id: String,
    pub alt: String,
    pub is_hero: bool,
}
pub struct ImagesBorrowed<'a> {
    pub id: &'a str,
    pub alt: &'a str,
    pub is_hero: bool,
}
impl<'a> From<ImagesBorrowed<'a>> for Images {
    fn from(ImagesBorrowed { id, alt, is_hero }: ImagesBorrowed<'a>) -> Self {
        Self {
            id: id.into(),
            alt: alt.into(),
            is_hero,
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
pub struct ImagesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ImagesBorrowed, tokio_postgres::Error>,
    mapper: fn(ImagesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ImagesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ImagesBorrowed) -> R) -> ImagesQuery<'c, 'a, 's, C, R, N> {
        ImagesQuery {
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
        "SELECT \"Product\".id, \"Product\".publish_at, \"Product\".slug, \"Product\".meta_title, \"Product\".meta_description, \"Product\".h1, \"Content\".id AS content_id, \"Content\".json AS content_json FROM \"Product\" INNER JOIN \"Content\" ON \"Content\".id = \"Product\".content_id WHERE slug = $1 AND publish_at < NOW()",
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
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        slug: &'a T1,
    ) -> PageQuery<'c, 'a, 's, C, Page, 1> {
        PageQuery {
            client,
            params: [slug],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<PageBorrowed, tokio_postgres::Error> {
                Ok(PageBorrowed {
                    id: row.try_get(0)?,
                    publish_at: row.try_get(1)?,
                    slug: row.try_get(2)?,
                    meta_title: row.try_get(3)?,
                    meta_description: row.try_get(4)?,
                    h1: row.try_get(5)?,
                    content_id: row.try_get(6)?,
                    content_json: row.try_get(7)?,
                })
            },
            mapper: |it| Page::from(it),
        }
    }
}
pub struct ListStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list() -> ListStmt {
    ListStmt(
        "SELECT \"Product\".id, \"Product\".publish_at, \"Product\".slug, \"Product\".h1, \"ProductImage\".id AS image_id, \"ProductImage\".alt FROM \"Product\" LEFT JOIN \"ProductImage\" ON \"Product\".id = \"ProductImage\".product_id AND \"ProductImage\".is_hero = true WHERE \"Product\".publish_at < NOW()",
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
                    image_id: row.try_get(4)?,
                    alt: row.try_get(5)?,
                })
            },
            mapper: |it| List::from(it),
        }
    }
}
pub struct ImagesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn images() -> ImagesStmt {
    ImagesStmt(
        "SELECT id, alt, is_hero FROM \"ProductImage\" WHERE product_id = $1",
        None,
    )
}
impl ImagesStmt {
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
        product_id: &'a T1,
    ) -> ImagesQuery<'c, 'a, 's, C, Images, 1> {
        ImagesQuery {
            client,
            params: [product_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ImagesBorrowed, tokio_postgres::Error> {
                    Ok(ImagesBorrowed {
                        id: row.try_get(0)?,
                        alt: row.try_get(1)?,
                        is_hero: row.try_get(2)?,
                    })
                },
            mapper: |it| Images::from(it),
        }
    }
}
