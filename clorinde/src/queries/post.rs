// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct PostCreateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub published_at: Option<crate::types::time::Timestamp>,
    pub title: T1,
    pub description: T2,
    pub text: T3,
}
#[derive(Debug)]
pub struct PostUpdateParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub published_at: Option<crate::types::time::Timestamp>,
    pub slug: T1,
    pub title: T2,
    pub description: T3,
    pub text: T4,
    pub id: T5,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PostPage {
    pub id: String,
    pub published_at: Option<crate::types::time::Timestamp>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub text: String,
}
pub struct PostPageBorrowed<'a> {
    pub id: &'a str,
    pub published_at: Option<crate::types::time::Timestamp>,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub text: &'a str,
}
impl<'a> From<PostPageBorrowed<'a>> for PostPage {
    fn from(
        PostPageBorrowed {
            id,
            published_at,
            slug,
            title,
            description,
            text,
        }: PostPageBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            published_at,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PostList {
    pub id: String,
    pub published_at: crate::types::time::Timestamp,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub text: String,
    pub image_id: Option<String>,
    pub alt: Option<String>,
}
pub struct PostListBorrowed<'a> {
    pub id: &'a str,
    pub published_at: crate::types::time::Timestamp,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub text: &'a str,
    pub image_id: Option<&'a str>,
    pub alt: Option<&'a str>,
}
impl<'a> From<PostListBorrowed<'a>> for PostList {
    fn from(
        PostListBorrowed {
            id,
            published_at,
            slug,
            title,
            description,
            text,
            image_id,
            alt,
        }: PostListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            published_at,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            text: text.into(),
            image_id: image_id.map(|v| v.into()),
            alt: alt.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AdminPostPage {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub published_at: Option<crate::types::time::Timestamp>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub text: String,
}
pub struct AdminPostPageBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub published_at: Option<crate::types::time::Timestamp>,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub text: &'a str,
}
impl<'a> From<AdminPostPageBorrowed<'a>> for AdminPostPage {
    fn from(
        AdminPostPageBorrowed {
            id,
            created_at,
            published_at,
            slug,
            title,
            description,
            text,
        }: AdminPostPageBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            published_at,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PostCreate {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
}
pub struct PostCreateBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
}
impl<'a> From<PostCreateBorrowed<'a>> for PostCreate {
    fn from(PostCreateBorrowed { id, created_at }: PostCreateBorrowed<'a>) -> Self {
        Self {
            id: id.into(),
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PostImages {
    pub id: String,
    pub alt: String,
    pub is_hero: bool,
}
pub struct PostImagesBorrowed<'a> {
    pub id: &'a str,
    pub alt: &'a str,
    pub is_hero: bool,
}
impl<'a> From<PostImagesBorrowed<'a>> for PostImages {
    fn from(PostImagesBorrowed { id, alt, is_hero }: PostImagesBorrowed<'a>) -> Self {
        Self {
            id: id.into(),
            alt: alt.into(),
            is_hero,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AdminList {
    pub id: String,
    pub created_at: crate::types::time::Timestamp,
    pub published_at: Option<crate::types::time::Timestamp>,
    pub title: String,
    pub image_id: Option<String>,
}
pub struct AdminListBorrowed<'a> {
    pub id: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub published_at: Option<crate::types::time::Timestamp>,
    pub title: &'a str,
    pub image_id: Option<&'a str>,
}
impl<'a> From<AdminListBorrowed<'a>> for AdminList {
    fn from(
        AdminListBorrowed {
            id,
            created_at,
            published_at,
            title,
            image_id,
        }: AdminListBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            created_at,
            published_at,
            title: title.into(),
            image_id: image_id.map(|v| v.into()),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct PostPageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<PostPageBorrowed, tokio_postgres::Error>,
    mapper: fn(PostPageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PostPageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(PostPageBorrowed) -> R) -> PostPageQuery<'c, 'a, 's, C, R, N> {
        PostPageQuery {
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
pub struct PostListQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<PostListBorrowed, tokio_postgres::Error>,
    mapper: fn(PostListBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PostListQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(PostListBorrowed) -> R) -> PostListQuery<'c, 'a, 's, C, R, N> {
        PostListQuery {
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
pub struct AdminPostPageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<AdminPostPageBorrowed, tokio_postgres::Error>,
    mapper: fn(AdminPostPageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AdminPostPageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(AdminPostPageBorrowed) -> R,
    ) -> AdminPostPageQuery<'c, 'a, 's, C, R, N> {
        AdminPostPageQuery {
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
pub struct PostCreateQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<PostCreateBorrowed, tokio_postgres::Error>,
    mapper: fn(PostCreateBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PostCreateQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(PostCreateBorrowed) -> R,
    ) -> PostCreateQuery<'c, 'a, 's, C, R, N> {
        PostCreateQuery {
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
pub struct PostImagesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<PostImagesBorrowed, tokio_postgres::Error>,
    mapper: fn(PostImagesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PostImagesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(PostImagesBorrowed) -> R,
    ) -> PostImagesQuery<'c, 'a, 's, C, R, N> {
        PostImagesQuery {
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
pub struct AdminListQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<AdminListBorrowed, tokio_postgres::Error>,
    mapper: fn(AdminListBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AdminListQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(AdminListBorrowed) -> R) -> AdminListQuery<'c, 'a, 's, C, R, N> {
        AdminListQuery {
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
pub fn post_page() -> PostPageStmt {
    PostPageStmt(crate::client::async_::Stmt::new(
        "SELECT id, published_at, slug, title, description, text FROM \"Post\" WHERE slug = $1 AND published_at < NOW()",
    ))
}
pub struct PostPageStmt(crate::client::async_::Stmt);
impl PostPageStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        slug: &'a T1,
    ) -> PostPageQuery<'c, 'a, 's, C, PostPage, 1> {
        PostPageQuery {
            client,
            params: [slug],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<PostPageBorrowed, tokio_postgres::Error> {
                    Ok(PostPageBorrowed {
                        id: row.try_get(0)?,
                        published_at: row.try_get(1)?,
                        slug: row.try_get(2)?,
                        title: row.try_get(3)?,
                        description: row.try_get(4)?,
                        text: row.try_get(5)?,
                    })
                },
            mapper: |it| PostPage::from(it),
        }
    }
}
pub fn post_list() -> PostListStmt {
    PostListStmt(crate::client::async_::Stmt::new(
        "SELECT \"Post\".\"id\", \"Post\".\"published_at\", \"Post\".\"slug\", \"Post\".\"title\", \"Post\".\"description\", \"Post\".\"text\", \"Image\".\"id\" AS \"image_id\", \"Image\".\"alt\" FROM \"Post\" INNER JOIN \"Image\" ON \"Image\".\"post_id\" = \"Post\".\"id\" WHERE \"Post\".\"published_at\" < NOW() AND \"Image\".\"is_hero\" = true LIMIT 10",
    ))
}
pub struct PostListStmt(crate::client::async_::Stmt);
impl PostListStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> PostListQuery<'c, 'a, 's, C, PostList, 0> {
        PostListQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<PostListBorrowed, tokio_postgres::Error> {
                    Ok(PostListBorrowed {
                        id: row.try_get(0)?,
                        published_at: row.try_get(1)?,
                        slug: row.try_get(2)?,
                        title: row.try_get(3)?,
                        description: row.try_get(4)?,
                        text: row.try_get(5)?,
                        image_id: row.try_get(6)?,
                        alt: row.try_get(7)?,
                    })
                },
            mapper: |it| PostList::from(it),
        }
    }
}
pub fn admin_post_page() -> AdminPostPageStmt {
    AdminPostPageStmt(crate::client::async_::Stmt::new(
        "SELECT id, created_at, published_at, slug, title, description, text FROM \"Post\" WHERE id = $1",
    ))
}
pub struct AdminPostPageStmt(crate::client::async_::Stmt);
impl AdminPostPageStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> AdminPostPageQuery<'c, 'a, 's, C, AdminPostPage, 1> {
        AdminPostPageQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<AdminPostPageBorrowed, tokio_postgres::Error> {
                    Ok(AdminPostPageBorrowed {
                        id: row.try_get(0)?,
                        created_at: row.try_get(1)?,
                        published_at: row.try_get(2)?,
                        slug: row.try_get(3)?,
                        title: row.try_get(4)?,
                        description: row.try_get(5)?,
                        text: row.try_get(6)?,
                    })
                },
            mapper: |it| AdminPostPage::from(it),
        }
    }
}
pub fn admin_post_by_slug() -> AdminPostBySlugStmt {
    AdminPostBySlugStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"Post\" WHERE slug = $1",
    ))
}
pub struct AdminPostBySlugStmt(crate::client::async_::Stmt);
impl AdminPostBySlugStmt {
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
pub fn admin_post_by_id_check() -> AdminPostByIdCheckStmt {
    AdminPostByIdCheckStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"Post\" WHERE id = $1",
    ))
}
pub struct AdminPostByIdCheckStmt(crate::client::async_::Stmt);
impl AdminPostByIdCheckStmt {
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
pub fn post_create() -> PostCreateStmt {
    PostCreateStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"Post\" (published_at, title, description, text) VALUES ($1, $2, $3, $4) RETURNING id, created_at",
    ))
}
pub struct PostCreateStmt(crate::client::async_::Stmt);
impl PostCreateStmt {
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
        published_at: &'a Option<crate::types::time::Timestamp>,
        title: &'a T1,
        description: &'a T2,
        text: &'a T3,
    ) -> PostCreateQuery<'c, 'a, 's, C, PostCreate, 4> {
        PostCreateQuery {
            client,
            params: [published_at, title, description, text],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<PostCreateBorrowed, tokio_postgres::Error> {
                    Ok(PostCreateBorrowed {
                        id: row.try_get(0)?,
                        created_at: row.try_get(1)?,
                    })
                },
            mapper: |it| PostCreate::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        PostCreateParams<T1, T2, T3>,
        PostCreateQuery<'c, 'a, 's, C, PostCreate, 4>,
        C,
    > for PostCreateStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a PostCreateParams<T1, T2, T3>,
    ) -> PostCreateQuery<'c, 'a, 's, C, PostCreate, 4> {
        self.bind(
            client,
            &params.published_at,
            &params.title,
            &params.description,
            &params.text,
        )
    }
}
pub fn post_update() -> PostUpdateStmt {
    PostUpdateStmt(crate::client::async_::Stmt::new(
        "UPDATE \"Post\" SET published_at = $1, slug = $2, title = $3, description = $4, text = $5 WHERE id = $6 RETURNING created_at",
    ))
}
pub struct PostUpdateStmt(crate::client::async_::Stmt);
impl PostUpdateStmt {
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
        published_at: &'a Option<crate::types::time::Timestamp>,
        slug: &'a T1,
        title: &'a T2,
        description: &'a T3,
        text: &'a T4,
        id: &'a T5,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 6> {
        CrateTypesTimeTimestampQuery {
            client,
            params: [published_at, slug, title, description, text, id],
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
        PostUpdateParams<T1, T2, T3, T4, T5>,
        CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 6>,
        C,
    > for PostUpdateStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a PostUpdateParams<T1, T2, T3, T4, T5>,
    ) -> CrateTypesTimeTimestampQuery<'c, 'a, 's, C, crate::types::time::Timestamp, 6> {
        self.bind(
            client,
            &params.published_at,
            &params.slug,
            &params.title,
            &params.description,
            &params.text,
            &params.id,
        )
    }
}
pub fn post_delete() -> PostDeleteStmt {
    PostDeleteStmt(crate::client::async_::Stmt::new(
        "DELETE FROM \"Post\" WHERE id = $1",
    ))
}
pub struct PostDeleteStmt(crate::client::async_::Stmt);
impl PostDeleteStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn post_images() -> PostImagesStmt {
    PostImagesStmt(crate::client::async_::Stmt::new(
        "SELECT id, alt, is_hero FROM \"Image\" WHERE post_id = $1",
    ))
}
pub struct PostImagesStmt(crate::client::async_::Stmt);
impl PostImagesStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        post_id: &'a T1,
    ) -> PostImagesQuery<'c, 'a, 's, C, PostImages, 1> {
        PostImagesQuery {
            client,
            params: [post_id],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<PostImagesBorrowed, tokio_postgres::Error> {
                    Ok(PostImagesBorrowed {
                        id: row.try_get(0)?,
                        alt: row.try_get(1)?,
                        is_hero: row.try_get(2)?,
                    })
                },
            mapper: |it| PostImages::from(it),
        }
    }
}
pub fn post_images_ids() -> PostImagesIdsStmt {
    PostImagesIdsStmt(crate::client::async_::Stmt::new(
        "SELECT id FROM \"Image\" WHERE post_id = $1",
    ))
}
pub struct PostImagesIdsStmt(crate::client::async_::Stmt);
impl PostImagesIdsStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        post_id: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [post_id],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub fn admin_list() -> AdminListStmt {
    AdminListStmt(crate::client::async_::Stmt::new(
        "SELECT \"Post\".\"id\", \"Post\".\"created_at\", \"Post\".\"published_at\", \"Post\".\"title\", \"Image\".\"id\" AS \"image_id\" FROM \"Post\" INNER JOIN \"Image\" ON \"Image\".\"post_id\" = \"Post\".\"id\" AND \"Image\".\"is_hero\" = true",
    ))
}
pub struct AdminListStmt(crate::client::async_::Stmt);
impl AdminListStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> AdminListQuery<'c, 'a, 's, C, AdminList, 0> {
        AdminListQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<AdminListBorrowed, tokio_postgres::Error> {
                    Ok(AdminListBorrowed {
                        id: row.try_get(0)?,
                        created_at: row.try_get(1)?,
                        published_at: row.try_get(2)?,
                        title: row.try_get(3)?,
                        image_id: row.try_get(4)?,
                    })
                },
            mapper: |it| AdminList::from(it),
        }
    }
}
