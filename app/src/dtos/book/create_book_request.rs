use async_graphql::{ID, InputObject};

#[derive(InputObject)]
pub struct CreateBookRequest {
    pub author_id: ID,

    pub title: String,

    pub published_year: i32,

    pub genre: String,
}
