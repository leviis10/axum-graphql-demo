use crate::dtos::book::{CreateBookRequest, UpdateBookRequest};
use crate::entities::books;
use crate::errors::{AppError, Result};
use crate::{handlers, repositories};
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel};
use time::OffsetDateTime;

pub async fn create(db: &DatabaseConnection, request: CreateBookRequest) -> Result<books::Model> {
    let found_author = handlers::author::get_by_id(db, request.author_id.parse()?).await?;

    let new_book = books::ActiveModel {
        author_id: ActiveValue::Set(found_author.id),
        title: ActiveValue::Set(request.title),
        published_year: ActiveValue::Set(request.published_year),
        genre: ActiveValue::Set(request.genre),
        ..Default::default()
    };

    repositories::book::save(db, new_book).await
}

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

pub async fn update_by_id(
    db: &DatabaseConnection,
    id: i32,
    request: UpdateBookRequest,
) -> Result<books::Model> {
    let mut found_book = get_by_id(db, id).await?.into_active_model();
    let author_id: i32 = request.author_id.parse()?;

    found_book.title = ActiveValue::Set(request.title);
    found_book.published_year = ActiveValue::Set(request.published_year);
    found_book.genre = ActiveValue::Set(request.genre);
    found_book.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());
    if author_id != found_book.clone().author_id.unwrap() {
        let found_author = handlers::author::get_by_id(db, author_id).await?;
        found_book.author_id = ActiveValue::Set(found_author.id);
    }

    repositories::book::save(db, found_book).await
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<bool> {
    let found_book = get_by_id(db, id).await?;

    repositories::book::delete(db, found_book).await?;
    Ok(true)
}
