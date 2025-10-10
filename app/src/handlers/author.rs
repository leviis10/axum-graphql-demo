use crate::dtos::author::{CreateAuthorRequest, UpdateAuthorRequest};
use crate::entities::authors;
use crate::errors::{AppError, Result};
use crate::repositories;
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel};
use time::OffsetDateTime;

pub async fn create(
    db: &DatabaseConnection,
    request: CreateAuthorRequest,
) -> Result<authors::Model> {
    let new_author_model = authors::ActiveModel {
        name: ActiveValue::Set(request.name),
        ..Default::default()
    };

    repositories::author::save(db, new_author_model).await
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<authors::Model> {
    let found_author = repositories::author::get_by_id(db, id).await?;
    let Some(author) = found_author else {
        return Err(AppError::NotFound(String::from("Author not found")));
    };
    Ok(author)
}

pub async fn find_authors(db: &DatabaseConnection) -> Result<Vec<authors::Model>> {
    repositories::author::find_all(db).await
}

pub async fn update_by_id(
    db: &DatabaseConnection,
    id: i32,
    request: UpdateAuthorRequest,
) -> Result<authors::Model> {
    let mut found_author = get_by_id(db, id).await?.into_active_model();
    found_author.name = ActiveValue::Set(request.name);
    found_author.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());

    let updated_author = repositories::author::save(db, found_author).await?;
    Ok(updated_author)
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
    let found_author = get_by_id(db, id).await?;

    repositories::author::delete(db, found_author).await?;

    Ok(())
}
