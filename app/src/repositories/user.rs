use crate::entities::prelude::Users;
use crate::entities::users;
use crate::errors::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TryIntoModel,
};

pub async fn save(
    connection: &impl ConnectionTrait,
    model: users::ActiveModel,
) -> Result<users::Model> {
    let registered_user = model.save(connection).await?.try_into_model()?;
    Ok(registered_user)
}

pub async fn get_one_by_username(
    connection: &impl ConnectionTrait,
    username: &str,
) -> Result<Option<users::Model>> {
    let found_user = Users::find()
        .filter(users::Column::Username.eq(username))
        .one(connection)
        .await?;

    Ok(found_user)
}
