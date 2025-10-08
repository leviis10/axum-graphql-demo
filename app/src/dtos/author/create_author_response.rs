use crate::entities::authors;
use crate::errors::AppError;
use crate::errors::Result;
use async_graphql::{ID, SimpleObject};
use time::format_description::well_known::Rfc3339;

#[derive(SimpleObject)]
pub struct CreateAuthorResponse {
    pub id: ID,

    pub name: String,

    pub created_at: String,

    pub updated_at: String,
}

impl TryFrom<authors::Model> for CreateAuthorResponse {
    type Error = AppError;

    fn try_from(model: authors::Model) -> Result<Self> {
        Ok(CreateAuthorResponse {
            id: ID(model.id.to_string()),
            name: model.name,
            created_at: model.created_at.format(&Rfc3339)?,
            updated_at: model.updated_at.format(&Rfc3339)?,
        })
    }
}
