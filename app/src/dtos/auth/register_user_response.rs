use crate::entities::users;
use crate::errors::AppError;
use crate::errors::Result;
use async_graphql::{ID, SimpleObject};
use time::format_description::well_known::Rfc3339;

#[derive(SimpleObject)]
pub struct RegisterUserResponse {
    pub id: ID,

    pub username: String,

    pub email: String,

    pub created_at: String,

    pub updated_at: String,
}

impl TryFrom<users::Model> for RegisterUserResponse {
    type Error = AppError;

    fn try_from(model: users::Model) -> Result<Self> {
        Ok(RegisterUserResponse {
            id: ID::from(model.id),
            username: model.username,
            email: model.email,
            created_at: model.created_at.format(&Rfc3339)?,
            updated_at: model.updated_at.format(&Rfc3339)?,
        })
    }
}
