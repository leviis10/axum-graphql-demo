use crate::dtos::author::{CreateAuthorRequest, CreateAuthorResponse};
use crate::errors::Result;
use crate::handlers;
use async_graphql::{Context, Object};
use sea_orm::DatabaseConnection;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_author<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        request: CreateAuthorRequest,
    ) -> Result<CreateAuthorResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        let new_author = handlers::author::create(db, request).await?;

        CreateAuthorResponse::try_from(new_author)
    }
}
