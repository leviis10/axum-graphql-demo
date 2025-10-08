use crate::entities::books;
use crate::errors::{AppError, Result};
use crate::repositories;
use sea_orm::DatabaseConnection;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<books::Model> {
    let found_book = repositories::book::get_by_id(db, id).await?;
    let Some(book) = found_book else {
        return Err(AppError::NotFound(String::from("Book not found")));
    };

    Ok(book)
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<books::Model>> {
    repositories::book::find_all(db).await
}

pub async fn find_all_by_author_id(
    db: &DatabaseConnection,
    author_id: i32,
) -> Result<Vec<books::Model>> {
    repositories::book::find_all_by_author_id(db, author_id).await
}
