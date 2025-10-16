use crate::dtos::refresh_token::GetRefreshTokenRequest;
use crate::entities::refresh_tokens;
use crate::errors::{AppError, Result};
use crate::repositories;
use crate::utils::jwt::RefreshToken;
use sea_orm::{ActiveValue, DatabaseConnection, DatabaseTransaction, IntoActiveModel};
use time::OffsetDateTime;

pub async fn create(db: &DatabaseTransaction, claims: &RefreshToken) -> Result<String> {
    let expires_at = OffsetDateTime::from_unix_timestamp(claims.exp as i64)?;
    let refresh_token = claims.generate()?;

    let model = refresh_tokens::ActiveModel {
        jti: ActiveValue::Set(claims.jti),
        user_id: ActiveValue::Set(claims.sub),
        hashed_token: ActiveValue::Set(RefreshToken::hash(&refresh_token)),
        expires_at: ActiveValue::Set(expires_at),
        ..Default::default()
    };
    repositories::refresh_token::insert(db, model).await?;

    Ok(refresh_token)
}

pub async fn get_active_one_by_pk_and_user_id_and_hashed_token(
    db: &DatabaseConnection,
    request: GetRefreshTokenRequest,
) -> Result<refresh_tokens::Model> {
    let found_token_option =
        repositories::refresh_token::get_active_one_by_pk_and_user_id_and_hashed_token(
            db,
            request.jti,
            request.user_id,
            &request.hashed_token,
        )
        .await?;
    let Some(found_token) = found_token_option else {
        return Err(AppError::NotFound(String::from("Refresh Token Not Found")));
    };
    Ok(found_token)
}

pub async fn revoke_using_model(
    db: &DatabaseTransaction,
    model: refresh_tokens::Model,
) -> Result<refresh_tokens::Model> {
    let mut refresh_token_model = model.into_active_model();
    refresh_token_model.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));

    repositories::refresh_token::save(db, refresh_token_model).await
}
