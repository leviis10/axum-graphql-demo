use crate::dtos::author::GetAuthorResponse;
use crate::entities::books;
use crate::errors::AppError;
use crate::errors::Result;
use crate::handlers;
use async_graphql::{ComplexObject, Context, ID, SimpleObject};
use sea_orm::DatabaseConnection;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UpdateBookResponse {
    pub id: ID,

    pub title: String,

    pub published_year: i32,

    pub genre: String,

    pub created_at: String,

    pub updated_at: String,

    #[graphql(skip)]
    pub author_id: i32,
}

#[ComplexObject]
impl UpdateBookResponse {
    pub async fn author<'ctx>(&self, ctx: &Context<'ctx>) -> Result<GetAuthorResponse> {
        let db: &DatabaseConnection = ctx.data()?;

        let found_author = handlers::author::get_by_id(db, self.author_id).await?;
        GetAuthorResponse::try_from(found_author)
    }
}

impl TryFrom<books::Model> for UpdateBookResponse {
    type Error = AppError;

    fn try_from(model: books::Model) -> Result<Self> {
        Ok(UpdateBookResponse {
            id: ID::from(model.id),
            title: model.title,
            published_year: model.published_year,
            genre: model.genre,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            author_id: model.author_id,
        })
    }
}
