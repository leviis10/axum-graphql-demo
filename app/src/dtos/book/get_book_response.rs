use crate::dtos::author::GetAuthorResponse;
use crate::entities::books;
use crate::errors::AppError;
use crate::errors::Result;
use crate::handlers;
use async_graphql::{ComplexObject, Context, ID, SimpleObject};
use sea_orm::DatabaseConnection;
use time::format_description::well_known::Rfc3339;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct GetBookResponse {
    pub id: ID,

    #[graphql(skip)]
    pub author_id: i32,

    pub title: String,

    pub published_year: i32,

    pub genre: String,

    pub created_at: String,

    pub updated_at: String,
}

#[ComplexObject]
impl GetBookResponse {
    async fn author<'ctx>(&self, ctx: &Context<'ctx>) -> Result<GetAuthorResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        let found_author = handlers::author::get_by_id(db, self.author_id).await?;

        GetAuthorResponse::try_from(found_author)
    }
}

impl TryFrom<books::Model> for GetBookResponse {
    type Error = AppError;

    fn try_from(model: books::Model) -> Result<Self> {
        Ok(GetBookResponse {
            id: ID::from(model.id),
            author_id: model.author_id,
            title: model.title,
            published_year: model.published_year,
            genre: model.genre,
            created_at: model.created_at.format(&Rfc3339)?,
            updated_at: model.updated_at.format(&Rfc3339)?,
        })
    }
}
