use async_graphql::InputObject;

#[derive(InputObject)]
pub struct UpdateAuthorRequest {
    pub name: String,
}
