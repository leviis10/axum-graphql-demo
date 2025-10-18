use crate::dtos::book::GetBookResponse;
use crate::entities::authors;
use crate::errors::{AppError, Result};
use crate::loaders::book::BookLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, ID, SimpleObject};
use time::format_description::well_known::Rfc3339;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct GetAuthorLoaderResponse {
    pub id: ID,

    pub name: String,

    pub created_at: String,

    pub updated_at: String,
}

#[ComplexObject]
impl GetAuthorLoaderResponse {
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<GetBookResponse>> {
        let loader: &DataLoader<BookLoader> = ctx.data()?;

        let found_books = loader
            .load_one(self.id.parse()?)
            .await
            .map_err(|_| AppError::LoaderError(String::from("Error from book loader")))?
            .unwrap_or_default();

        Ok(found_books)
    }
}

impl TryFrom<authors::Model> for GetAuthorLoaderResponse {
    type Error = AppError;

    fn try_from(model: authors::Model) -> Result<Self> {
        Ok(Self {
            id: ID::from(model.id),
            name: model.name,
            created_at: model.created_at.format(&Rfc3339)?,
            updated_at: model.updated_at.format(&Rfc3339)?,
        })
    }
}
