use crate::dtos::author::{GetAuthorLoaderResponse, GetAuthorResponse};
use crate::dtos::book::GetBookResponse;
use crate::errors::Result;
use crate::handlers;
use async_graphql::{Context, ID, Object};
use sea_orm::DatabaseConnection;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn author<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<GetAuthorResponse> {
        let connection = ctx.data::<DatabaseConnection>()?;

        let found_author = handlers::author::get_by_id(connection, id.parse()?).await?;

        GetAuthorResponse::try_from(found_author)
    }

    async fn authors<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<GetAuthorResponse>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let found_authors = handlers::author::find_authors(db).await?;

        found_authors
            .into_iter()
            .map(GetAuthorResponse::try_from)
            .collect()
    }

    async fn authors_loader(&self, ctx: &Context<'_>) -> Result<Vec<GetAuthorLoaderResponse>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let found_authors = handlers::author::find_authors(db).await?;

        found_authors
            .into_iter()
            .map(GetAuthorLoaderResponse::try_from)
            .collect()
    }

    async fn book<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<GetBookResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        let found_book = handlers::book::get_by_id(db, id.parse()?).await?;

        GetBookResponse::try_from(found_book)
    }

    async fn books<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<GetBookResponse>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let found_books = handlers::book::find_all(db).await?;

        found_books
            .into_iter()
            .map(GetBookResponse::try_from)
            .collect()
    }
}
