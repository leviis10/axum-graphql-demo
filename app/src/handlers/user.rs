use crate::dtos::user::CreateUserRequest;
use crate::entities::users;
use crate::errors::{AppError, Result};
use crate::repositories;
use sea_orm::{ActiveValue, DatabaseConnection, DatabaseTransaction};

pub async fn create(txn: &DatabaseTransaction, request: CreateUserRequest) -> Result<users::Model> {
    let user_model = users::ActiveModel {
        username: ActiveValue::Set(request.username),
        email: ActiveValue::Set(request.email),
        password: ActiveValue::Set(request.hashed_password),
        ..Default::default()
    };

    repositories::user::save(txn, user_model).await
}

pub async fn get_one_by_username(db: &DatabaseConnection, username: &str) -> Result<users::Model> {
    let found_user_option = repositories::user::get_one_by_username(db, username).await?;
    let Some(found_user) = found_user_option else {
        return Err(AppError::NotFound(String::from("User not found")));
    };
    Ok(found_user)
}
