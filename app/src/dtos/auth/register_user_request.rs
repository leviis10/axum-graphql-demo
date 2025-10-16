use async_graphql::InputObject;

#[derive(InputObject)]
pub struct RegisterUserRequest {
    pub email: String,

    pub username: String,

    pub password: String,
}
