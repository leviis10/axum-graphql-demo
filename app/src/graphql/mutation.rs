use crate::dtos::auth::{
    LoginRequest, LoginResponse, RefreshTokenResponse, RegisterUserRequest, RegisterUserResponse,
};
use crate::dtos::author::{
    CreateAuthorRequest, CreateAuthorResponse, UpdateAuthorRequest, UpdateAuthorResponse,
};
use crate::dtos::book::{
    CreateBookRequest, CreateBookResponse, UpdateBookRequest, UpdateBookResponse,
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

    async fn create_book<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        request: CreateBookRequest,
    ) -> Result<CreateBookResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        let new_book = handlers::book::create(db, request).await?;

        CreateBookResponse::try_from(new_book)
    }

    async fn update_book_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
        request: UpdateBookRequest,
    ) -> Result<UpdateBookResponse> {
        let db: &DatabaseConnection = ctx.data()?;

        let updated_book = handlers::book::update_by_id(db, id, request).await?;
        UpdateBookResponse::try_from(updated_book)
    }

    async fn delete_book_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let db: &DatabaseConnection = ctx.data()?;

        handlers::book::delete_by_id(db, id.parse()?).await?;
        Ok(true)
    }

    async fn register_user(
        &self,
        ctx: &Context<'_>,
        request: RegisterUserRequest,
    ) -> Result<RegisterUserResponse> {
        let db: &DatabaseConnection = ctx.data()?;

        let registered_user = handlers::auth::register(db, request).await?;

        RegisterUserResponse::try_from(registered_user)
    }

    async fn login(&self, ctx: &Context<'_>, request: LoginRequest) -> Result<LoginResponse> {
        let db: &DatabaseConnection = ctx.data()?;

        handlers::auth::login(db, request).await
    }

    async fn refresh_token(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> Result<RefreshTokenResponse> {
        let db: &DatabaseConnection = ctx.data()?;

        handlers::auth::refresh(db, &refresh_token).await
    }
}
