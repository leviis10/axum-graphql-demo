use crate::entities::books;
use crate::entities::prelude::Books;
use crate::errors::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, ModelTrait, QueryFilter, TryIntoModel};
use sea_orm::{ConnectionTrait, EntityTrait};

pub async fn save(
    connection: &impl ConnectionTrait,
    model: books::ActiveModel,
) -> Result<books::Model> {
    let new_book = model.save(connection).await?.try_into_model()?;
    Ok(new_book)
}

pub async fn get_by_id(connection: &impl ConnectionTrait, id: i32) -> Result<Option<books::Model>> {
    let found_book = Books::find_by_id(id).one(connection).await?;
    Ok(found_book)
}

pub async fn find_all(connection: &impl ConnectionTrait) -> Result<Vec<books::Model>> {
    let found_books = Books::find().all(connection).await?;
    Ok(found_books)
}

pub async fn find_all_by_author_id(
    connection: &impl ConnectionTrait,
    author_id: i32,
) -> Result<Vec<books::Model>> {
    let found_books = Books::find()
        .filter(books::Column::AuthorId.eq(author_id))
        .all(connection)
        .await?;
    Ok(found_books)
}

pub async fn delete(connection: &impl ConnectionTrait, model: books::Model) -> Result<bool> {
    model.delete(connection).await?;
    Ok(true)
}
