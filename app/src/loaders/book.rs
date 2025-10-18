use crate::dtos::book::GetBookResponse;
use crate::errors::AppError;
use crate::handlers;
use async_graphql::dataloader::Loader;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::sync::Arc;

pub struct BookLoader {
    pub db: DatabaseConnection,
}

impl Loader<i32> for BookLoader {
    type Value = Vec<GetBookResponse>;
    type Error = Arc<AppError>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let found_books = handlers::book::find_all_by_author_id_in(&self.db, keys).await?;

        let mut result = HashMap::new();
        for book in found_books {
            result
                .entry(book.author_id)
                .or_insert_with(Vec::new)
                .push(GetBookResponse::try_from(book)?);
        }
        Ok(result)
    }
}
