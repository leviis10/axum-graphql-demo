use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct RefreshTokenResponse {
    pub access_token: String,

    pub refresh_token: String,
}
