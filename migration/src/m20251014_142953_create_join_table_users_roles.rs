use crate::m20251014_131151_create_users_table::Users;
use crate::m20251014_141128_create_roles_table::Roles;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UsersRoles::Table)
                    .if_not_exists()
                    .col(integer(UsersRoles::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users-roles_users_user-id")
                            .from(UsersRoles::Table, UsersRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .col(integer(UsersRoles::RoleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users-roles_roles_role-id")
                            .from(UsersRoles::Table, UsersRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-user-id_role-id")
                            .col(UsersRoles::UserId)
                            .col(UsersRoles::RoleId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UsersRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UsersRoles {
    Table,
    UserId,
    RoleId,
}
