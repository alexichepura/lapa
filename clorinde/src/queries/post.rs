// This file was generated with `clorinde`. Do not modify.

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
