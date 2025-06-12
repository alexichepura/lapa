// This file was generated with `clorinde`. Do not modify.

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
