use crate::entities::users_roles;
use crate::errors::Result;
use sea_orm::{ActiveModelTrait, ConnectionTrait, TryIntoModel};

pub async fn insert(
    connection: &impl ConnectionTrait,
    model: users_roles::ActiveModel,
) -> Result<users_roles::Model> {
    let created_users_roles = model.insert(connection).await?.try_into_model()?;
    Ok(created_users_roles)
}
