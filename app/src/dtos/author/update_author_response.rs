use crate::entities::authors;
use crate::errors::AppError;
use async_graphql::{ID, SimpleObject};
use time::format_description::well_known::Rfc3339;

#[derive(SimpleObject)]
pub struct UpdateAuthorResponse {
    pub id: ID,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl TryFrom<authors::Model> for UpdateAuthorResponse {
    type Error = AppError;

    fn try_from(model: authors::Model) -> Result<Self, Self::Error> {
        Ok(UpdateAuthorResponse {
            id: ID::from(model.id),
            name: model.name,
            created_at: model.created_at.format(&Rfc3339)?,
            updated_at: model.updated_at.format(&Rfc3339)?,
        })
    }
}
