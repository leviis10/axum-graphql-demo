use crate::entities::prelude::RefreshTokens;
use crate::entities::refresh_tokens;
use crate::errors::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TryIntoModel,
};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn insert(
    connection: &impl ConnectionTrait,
    model: refresh_tokens::ActiveModel,
) -> Result<refresh_tokens::Model> {
    let inserted = model.insert(connection).await?;
    Ok(inserted)
}

pub async fn get_active_one_by_pk_and_user_id_and_hashed_token(
    connection: &impl ConnectionTrait,
    jti: Uuid,
    user_id: i32,
    hashed_token: &str,
) -> Result<Option<refresh_tokens::Model>> {
    let today = OffsetDateTime::now_utc();

    let found_token = RefreshTokens::find_by_id(jti)
        .filter(refresh_tokens::Column::UserId.eq(user_id))
        .filter(refresh_tokens::Column::HashedToken.eq(hashed_token))
        .filter(refresh_tokens::Column::DeletedAt.is_null())
        .filter(refresh_tokens::Column::ExpiresAt.gt(today))
        .one(connection)
        .await?;
    Ok(found_token)
}

pub async fn save(
    connection: &impl ConnectionTrait,
    model: refresh_tokens::ActiveModel,
) -> Result<refresh_tokens::Model> {
    let saved = model.save(connection).await?.try_into_model()?;
    Ok(saved)
}
