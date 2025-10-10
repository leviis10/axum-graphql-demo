use crate::dtos::book::GetBookResponse;
use crate::entities::authors;
use crate::errors::AppError;
use crate::errors::Result;
use crate::handlers;
use async_graphql::{ComplexObject, Context, ID, SimpleObject};
use sea_orm::DatabaseConnection;
use time::format_description::well_known::Rfc3339;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct GetAuthorResponse {
    pub id: ID,

    pub name: String,

    pub created_at: String,

    pub updated_at: String,
}

#[ComplexObject]
impl GetAuthorResponse {
    async fn books<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<GetBookResponse>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let found_books = handlers::book::find_all_by_author_id(db, self.id.parse()?).await?;

        found_books
            .into_iter()
            .map(GetBookResponse::try_from)
            .collect()
    }
}

impl TryFrom<authors::Model> for GetAuthorResponse {
    type Error = AppError;

    fn try_from(model: authors::Model) -> Result<Self> {
        Ok(GetAuthorResponse {
            id: ID::from(model.id),
            name: model.name,
            created_at: model.created_at.format(&Rfc3339)?,
            updated_at: model.updated_at.format(&Rfc3339)?,
        })
    }
}
