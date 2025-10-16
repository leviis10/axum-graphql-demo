use crate::m20251014_131151_create_users_table::Users;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RefreshTokens::Table)
                    .if_not_exists()
                    .col(pk_uuid(RefreshTokens::Jti))
                    .col(integer(RefreshTokens::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_refresh-tokens_users_user-id")
                            .from(RefreshTokens::Table, RefreshTokens::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .col(string(RefreshTokens::HashedToken))
                    .col(timestamp_with_time_zone(RefreshTokens::ExpiresAt))
                    .col(
                        timestamp_with_time_zone(RefreshTokens::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(RefreshTokens::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RefreshTokens {
    Table,
    Jti,
    UserId,
    HashedToken,
    ExpiresAt,
    CreatedAt,
    DeletedAt,
}
