use crate::dtos::author::{
    CreateAuthorRequest, CreateAuthorResponse, UpdateAuthorRequest, UpdateAuthorResponse,
};
use crate::errors::Result;
use crate::handlers;
use async_graphql::{Context, ID, Object};
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

    async fn update_author_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: ID,
        request: UpdateAuthorRequest,
    ) -> Result<UpdateAuthorResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        let updated_author = handlers::author::update_by_id(db, id.parse()?, request).await?;

        UpdateAuthorResponse::try_from(updated_author)
    }

    async fn delete_author_by_id<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;

        handlers::author::delete_by_id(db, id.parse()?).await?;

        Ok(true)
    }
}
