use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateAuthorRequest {
    pub name: String,
}
