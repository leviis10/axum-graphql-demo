use async_graphql::InputObject;

#[derive(InputObject)]
pub struct LoginRequest {
    pub username: String,

    pub password: String,
}
