use uuid::Uuid;

pub struct GetRefreshTokenRequest {
    pub jti: Uuid,
    pub user_id: i32,
    pub hashed_token: String,
}
