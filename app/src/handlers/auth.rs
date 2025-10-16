use crate::dtos::auth::{LoginRequest, LoginResponse, RefreshTokenResponse, RegisterUserRequest};
use crate::dtos::refresh_token::GetRefreshTokenRequest;
use crate::dtos::user::CreateUserRequest;
use crate::dtos::users_roles::AssignRoleRequest;
use crate::entities::users;
use crate::errors::{AppError, Result};
use crate::utils::jwt::{AccessToken, RefreshToken};
use crate::utils::password::compare_password;
use crate::{handlers, utils};
use sea_orm::{DatabaseConnection, TransactionTrait};

pub async fn register(
    db: &DatabaseConnection,
    request: RegisterUserRequest,
) -> Result<users::Model> {
    let txn = db.begin().await?;
    let hashed_password = utils::password::hash_password(request.password.as_bytes())?;

    tracing::info!("Searching for roles");
    let found_roles = handlers::role::find_by_name(db, vec![String::from("User")]).await?;
    tracing::debug!("found roles: {found_roles:?}");

    tracing::info!("Saving user");
    let new_user = handlers::user::create(
        &txn,
        CreateUserRequest {
            username: request.username,
            email: request.email,
            hashed_password,
        },
    )
    .await?;
    tracing::debug!("user saved: {new_user:?}");

    for role in found_roles.iter() {
        tracing::info!("Assigning role");
        handlers::users_roles::assign_role(
            &txn,
            AssignRoleRequest {
                user_id: new_user.id,
                role_id: role.id,
            },
        )
        .await?;
    }

    txn.commit().await?;
    Ok(new_user)
}

pub async fn login(db: &DatabaseConnection, request: LoginRequest) -> Result<LoginResponse> {
    let txn = db.begin().await?;
    let found_user = handlers::user::get_one_by_username(db, &request.username)
        .await
        .map_err(|_| {
            AppError::IncorrectCredentials(String::from("Incorrect username or password"))
        })?;
    compare_password(&request.password, &found_user.password).map_err(|_| {
        AppError::IncorrectCredentials(String::from("Incorrect username or password"))
    })?;

    let refresh_token_claim = RefreshToken::new(found_user.id)?;
    let refresh_token = handlers::refresh_token::create(&txn, &refresh_token_claim).await?;

    txn.commit().await?;
    Ok(LoginResponse {
        access_token: AccessToken::new(found_user.id)?.generate()?,
        refresh_token,
    })
}

pub async fn refresh(db: &DatabaseConnection, refresh_token: &str) -> Result<RefreshTokenResponse> {
    let txn = db.begin().await?;
    let hashed_refresh_token = RefreshToken::hash(refresh_token);
    let refresh_token_claim = RefreshToken::parse(refresh_token.as_bytes())?;

    let found_refresh_token =
        handlers::refresh_token::get_active_one_by_pk_and_user_id_and_hashed_token(
            db,
            GetRefreshTokenRequest {
                jti: refresh_token_claim.jti,
                user_id: refresh_token_claim.sub,
                hashed_token: hashed_refresh_token,
            },
        )
        .await?;

    handlers::refresh_token::revoke_using_model(&txn, found_refresh_token).await?;

    let new_access_token = AccessToken::new(refresh_token_claim.sub)?;
    let new_refresh_token = RefreshToken::new(refresh_token_claim.sub)?;
    handlers::refresh_token::create(&txn, &new_refresh_token).await?;

    txn.commit().await?;
    Ok(RefreshTokenResponse {
        access_token: new_access_token.generate()?,
        refresh_token: new_refresh_token.generate()?,
    })
}
