use crate::m20251014_141128_create_roles_table::Roles;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let initial_roles = vec!["Admin", "User"];

        for role in initial_roles.into_iter() {
            let insert_statement = Query::insert()
                .into_table(Roles::Table)
                .columns([Roles::Name])
                .values_panic([role.into()])
                .to_owned();
            manager.exec_stmt(insert_statement).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let initial_roles = vec!["Admin", "User"];

        for role in initial_roles.into_iter() {
            let delete_statement = Query::delete()
                .from_table(Roles::Table)
                .cond_where(any![Expr::col(Roles::Name).eq(role),])
                .to_owned();
            manager.exec_stmt(delete_statement).await?;
        }

        Ok(())
    }
}
