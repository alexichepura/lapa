// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct ProductCreateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub id: T1,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub meta_title: T2,
    pub meta_description: T3,
}
#[derive(Debug)]
pub struct ProductUpdateParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub slug: T1,
    pub meta_title: T2,
    pub meta_description: T3,
    pub id: T4,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProductPage {
    pub id: String,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
}
pub struct ProductPageBorrowed<'a> {
    pub id: &'a str,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub slug: &'a str,
    pub meta_title: &'a str,
    pub meta_description: &'a str,
}
impl<'a> From<ProductPageBorrowed<'a>> for ProductPage {
    fn from(
        ProductPageBorrowed {
            id,
            publish_at,
            slug,
            meta_title,
            meta_description,
        }: ProductPageBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            publish_at,
            slug: slug.into(),
            meta_title: meta_title.into(),
            meta_description: meta_description.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProductList {
    pub id: String,
    pub publish_at: crate::types::time::Timestamp,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
    pub image_id: Option<String>,
    pub alt: Option<String>,
}
pub struct ProductListBorrowed<'a> {
    pub id: &'a str,
    pub publish_at: crate::types::time::Timestamp,
    pub slug: &'a str,
    pub meta_title: &'a str,
    pub meta_description: &'a str,
    pub image_id: Option<&'a str>,
    pub alt: Option<&'a str>,
}
impl<'a> From<ProductListBorrowed<'a>> for ProductList {
    fn from(
        ProductListBorrowed {
            id,
            publish_at,
            slug,
            meta_title,
            meta_description,
            image_id,
            alt,
        }: ProductListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            publish_at,
            slug: slug.into(),
            meta_title: meta_title.into(),
            meta_description: meta_description.into(),
            image_id: image_id.map(|v| v.into()),
            alt: alt.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AdminProductPage {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub slug: String,
    pub meta_title: String,
    pub meta_description: String,
}
pub struct AdminProductPageBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub slug: &'a str,
    pub meta_title: &'a str,
    pub meta_description: &'a str,
}
impl<'a> From<AdminProductPageBorrowed<'a>> for AdminProductPage {
    fn from(
        AdminProductPageBorrowed {
            id,
            created_at,
            publish_at,
            slug,
            meta_title,
            meta_description,
        }: AdminProductPageBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            publish_at,
            slug: slug.into(),
            meta_title: meta_title.into(),
            meta_description: meta_description.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProductImages {
    pub id: String,
    pub alt: String,
    pub is_hero: bool,
}
pub struct ProductImagesBorrowed<'a> {
    pub id: &'a str,
    pub alt: &'a str,
    pub is_hero: bool,
}
impl<'a> From<ProductImagesBorrowed<'a>> for ProductImages {
    fn from(ProductImagesBorrowed { id, alt, is_hero }: ProductImagesBorrowed<'a>) -> Self {
        Self {
            id: id.into(),
            alt: alt.into(),
            is_hero,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AdminProductImages {
    pub id: String,
    pub alt: String,
    pub order: i32,
    pub is_hero: bool,
}
pub struct AdminProductImagesBorrowed<'a> {
    pub id: &'a str,
    pub alt: &'a str,
    pub order: i32,
    pub is_hero: bool,
}
impl<'a> From<AdminProductImagesBorrowed<'a>> for AdminProductImages {
    fn from(
        AdminProductImagesBorrowed {
            id,
            alt,
            order,
            is_hero,
        }: AdminProductImagesBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            alt: alt.into(),
            order,
            is_hero,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AdminProductList {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub meta_title: String,
    pub image_id: Option<String>,
}
pub struct AdminProductListBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub publish_at: Option<crate::types::time::Timestamp>,
    pub meta_title: &'a str,
    pub image_id: Option<&'a str>,
}
impl<'a> From<AdminProductListBorrowed<'a>> for AdminProductList {
    fn from(
        AdminProductListBorrowed {
            id,
            created_at,
            publish_at,
            meta_title,
            image_id,
        }: AdminProductListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            publish_at,
            meta_title: meta_title.into(),
            image_id: image_id.map(|v| v.into()),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ProductPageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<ProductPageBorrowed, tokio_postgres::Error>,
    mapper: fn(ProductPageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProductPageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProductPageBorrowed) -> R,
    ) -> ProductPageQuery<'c, 'a, 's, C, R, N> {
        ProductPageQuery {
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
pub struct ProductListQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<ProductListBorrowed, tokio_postgres::Error>,
    mapper: fn(ProductListBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProductListQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProductListBorrowed) -> R,
    ) -> ProductListQuery<'c, 'a, 's, C, R, N> {
        ProductListQuery {
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
pub struct AdminProductPageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<AdminProductPageBorrowed, tokio_postgres::Error>,
    mapper: fn(AdminProductPageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AdminProductPageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(AdminProductPageBorrowed) -> R,
    ) -> AdminProductPageQuery<'c, 'a, 's, C, R, N> {
        AdminProductPageQuery {
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
pub struct ProductImagesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<ProductImagesBorrowed, tokio_postgres::Error>,
    mapper: fn(ProductImagesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProductImagesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProductImagesBorrowed) -> R,
    ) -> ProductImagesQuery<'c, 'a, 's, C, R, N> {
        ProductImagesQuery {
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
pub struct AdminProductImagesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor:
        fn(&tokio_postgres::Row) -> Result<AdminProductImagesBorrowed, tokio_postgres::Error>,
    mapper: fn(AdminProductImagesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AdminProductImagesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(AdminProductImagesBorrowed) -> R,
    ) -> AdminProductImagesQuery<'c, 'a, 's, C, R, N> {
        AdminProductImagesQuery {
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
pub struct AdminProductListQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<AdminProductListBorrowed, tokio_postgres::Error>,
    mapper: fn(AdminProductListBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AdminProductListQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(AdminProductListBorrowed) -> R,
    ) -> AdminProductListQuery<'c, 'a, 's, C, R, N> {
        AdminProductListQuery {
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
pub fn product_page() -> ProductPageStmt {
    ProductPageStmt(crate::client::async_::Stmt::new(
        "SELECT id, publish_at, slug, meta_title, meta_description FROM \"Product\" WHERE slug = $1 AND publish_at < NOW()",
    ))
}
pub struct ProductPageStmt(crate::client::async_::Stmt);
impl ProductPageStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        slug: &'a T1,
    ) -> ProductPageQuery<'c, 'a, 's, C, ProductPage, 1> {
        ProductPageQuery {
            client,
            params: [slug],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProductPageBorrowed, tokio_postgres::Error> {
                    Ok(ProductPageBorrowed {
                        id: row.try_get(0)?,
                        publish_at: row.try_get(1)?,
                        slug: row.try_get(2)?,
                        meta_title: row.try_get(3)?,
                        meta_description: row.try_get(4)?,
                    })
                },
            mapper: |it| ProductPage::from(it),
        }
    }
}
pub fn product_list() -> ProductListStmt {
    ProductListStmt(crate::client::async_::Stmt::new(
        "SELECT \"Product\".\"id\", \"Product\".\"publish_at\", \"Product\".\"slug\", \"Product\".\"meta_title\", \"Product\".\"meta_description\", \"ProductImage\".\"id\" AS \"image_id\", \"ProductImage\".\"alt\" FROM \"Product\" INNER JOIN \"ProductImage\" ON \"ProductImage\".\"product_id\" = \"Product\".\"id\" WHERE \"Product\".\"publish_at\" < NOW() AND \"ProductImage\".\"is_hero\" = true LIMIT 10",
    ))
}
pub struct ProductListStmt(crate::client::async_::Stmt);
impl ProductListStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> ProductListQuery<'c, 'a, 's, C, ProductList, 0> {
        ProductListQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProductListBorrowed, tokio_postgres::Error> {
                    Ok(ProductListBorrowed {
                        id: row.try_get(0)?,
                        publish_at: row.try_get(1)?,
                        slug: row.try_get(2)?,
                        meta_title: row.try_get(3)?,
                        meta_description: row.try_get(4)?,
                        image_id: row.try_get(5)?,
                        alt: row.try_get(6)?,
                    })
                },
            mapper: |it| ProductList::from(it),
        }
    }
}
pub fn admin_product_page() -> AdminProductPageStmt {
    AdminProductPageStmt(crate::client::async_::Stmt::new(
        "SELECT id, created_at, publish_at, slug, meta_title, meta_description FROM \"Product\" WHERE id = $1",
    ))
}
pub struct AdminProductPageStmt(crate::client::async_::Stmt);
impl AdminProductPageStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> AdminProductPageQuery<'c, 'a, 's, C, AdminProductPage, 1> {
        AdminProductPageQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<AdminProductPageBorrowed, tokio_postgres::Error> {
                Ok(AdminProductPageBorrowed {
                    id: row.try_get(0)?,
                    created_at: row.try_get(1)?,
                    publish_at: row.try_get(2)?,
                    slug: row.try_get(3)?,
                    meta_title: row.try_get(4)?,
                    meta_description: row.try_get(5)?,
                })
            },
            mapper: |it| AdminProductPage::from(it),
        }
    }
}
pub fn admin_product_by_slug() -> AdminProductBySlugStmt {
    AdminProductBySlugStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"Product\" WHERE slug = $1",
    ))
}
pub struct AdminProductBySlugStmt(crate::client::async_::Stmt);
impl AdminProductBySlugStmt {
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
pub fn admin_product_by_id_check() -> AdminProductByIdCheckStmt {
    AdminProductByIdCheckStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"Product\" WHERE id = $1",
    ))
}
pub struct AdminProductByIdCheckStmt(crate::client::async_::Stmt);
impl AdminProductByIdCheckStmt {
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
pub fn product_create() -> ProductCreateStmt {
    ProductCreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"Product\" (id, publish_at, meta_title, meta_description) VALUES ($1, $2, $3, $4) RETURNING created_at",
    ))
}
pub struct ProductCreateStmt(crate::client::async_::Stmt);
impl ProductCreateStmt {
    pub fn bind<
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
        id: &'a T1,
        publish_at: &'a Option<crate::types::time::Timestamp>,
        meta_title: &'a T2,
        meta_description: &'a T3,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 4> {
        CrateTypesTimeTimestampQuery {
            client,
            params: [id, publish_at, meta_title, meta_description],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ProductCreateParams<T1, T2, T3>,
        CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 4>,
        C,
    > for ProductCreateStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a ProductCreateParams<T1, T2, T3>,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 4> {
        self.bind(
            client,
            &params.id,
            &params.publish_at,
            &params.meta_title,
            &params.meta_description,
        )
    }
}
pub fn product_update() -> ProductUpdateStmt {
    ProductUpdateStmt(crate::client::async_::Stmt::new(
        "UPDATE \"Product\" SET publish_at = $1, slug = $2, meta_title = $3, meta_description = $4 WHERE id = $5 RETURNING created_at",
    ))
}
pub struct ProductUpdateStmt(crate::client::async_::Stmt);
impl ProductUpdateStmt {
    pub fn bind<
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
        publish_at: &'a Option<crate::types::time::Timestamp>,
        slug: &'a T1,
        meta_title: &'a T2,
        meta_description: &'a T3,
        id: &'a T4,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 5> {
        CrateTypesTimeTimestampQuery {
            client,
            params: [publish_at, slug, meta_title, meta_description, id],
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
>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ProductUpdateParams<T1, T2, T3, T4>,
        CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 5>,
        C,
    > for ProductUpdateStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a ProductUpdateParams<T1, T2, T3, T4>,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 5> {
        self.bind(
            client,
            &params.publish_at,
            &params.slug,
            &params.meta_title,
            &params.meta_description,
            &params.id,
        )
    }
}
pub fn product_delete() -> ProductDeleteStmt {
    ProductDeleteStmt(crate::client::async_::Stmt::new(
        "DELETE FROM \"Product\" WHERE id = $1",
    ))
}
pub struct ProductDeleteStmt(crate::client::async_::Stmt);
impl ProductDeleteStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn product_images() -> ProductImagesStmt {
    ProductImagesStmt(crate::client::async_::Stmt::new(
        "SELECT id, alt, is_hero FROM \"ProductImage\" WHERE product_id = $1",
    ))
}
pub struct ProductImagesStmt(crate::client::async_::Stmt);
impl ProductImagesStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        product_id: &'a T1,
    ) -> ProductImagesQuery<'c, 'a, 's, C, ProductImages, 1> {
        ProductImagesQuery {
            client,
            params: [product_id],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProductImagesBorrowed, tokio_postgres::Error> {
                    Ok(ProductImagesBorrowed {
                        id: row.try_get(0)?,
                        alt: row.try_get(1)?,
                        is_hero: row.try_get(2)?,
                    })
                },
            mapper: |it| ProductImages::from(it),
        }
    }
}
pub fn admin_product_images() -> AdminProductImagesStmt {
    AdminProductImagesStmt(crate::client::async_::Stmt::new(
        "SELECT id, alt, \"order\", is_hero FROM \"ProductImage\" WHERE product_id = $1 ORDER BY \"order\"",
    ))
}
pub struct AdminProductImagesStmt(crate::client::async_::Stmt);
impl AdminProductImagesStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        product_id: &'a T1,
    ) -> AdminProductImagesQuery<'c, 'a, 's, C, AdminProductImages, 1> {
        AdminProductImagesQuery {
            client,
            params: [product_id],
            stmt: &mut self.0,
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<AdminProductImagesBorrowed, tokio_postgres::Error> {
                Ok(AdminProductImagesBorrowed {
                    id: row.try_get(0)?,
                    alt: row.try_get(1)?,
                    order: row.try_get(2)?,
                    is_hero: row.try_get(3)?,
                })
            },
            mapper: |it| AdminProductImages::from(it),
        }
    }
}
pub fn product_images_ids() -> ProductImagesIdsStmt {
    ProductImagesIdsStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"ProductImage\" WHERE product_id = $1",
    ))
}
pub struct ProductImagesIdsStmt(crate::client::async_::Stmt);
impl ProductImagesIdsStmt {
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
pub fn admin_product_list() -> AdminProductListStmt {
    AdminProductListStmt(crate::client::async_::Stmt::new(
        "SELECT \"Product\".\"id\", \"Product\".\"created_at\", \"Product\".\"publish_at\", \"Product\".\"meta_title\", \"ProductImage\".\"id\" AS \"image_id\" FROM \"Product\" INNER JOIN \"ProductImage\" ON \"ProductImage\".\"product_id\" = \"Product\".\"id\" AND \"ProductImage\".\"is_hero\" = true",
    ))
}
pub struct AdminProductListStmt(crate::client::async_::Stmt);
impl AdminProductListStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> AdminProductListQuery<'c, 'a, 's, C, AdminProductList, 0> {
        AdminProductListQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<AdminProductListBorrowed, tokio_postgres::Error> {
                Ok(AdminProductListBorrowed {
                    id: row.try_get(0)?,
                    created_at: row.try_get(1)?,
                    publish_at: row.try_get(2)?,
                    meta_title: row.try_get(3)?,
                    image_id: row.try_get(4)?,
                })
            },
            mapper: |it| AdminProductList::from(it),
        }
    }
}
