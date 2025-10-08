use crate::m20251007_165314_create_authors_table::Authors;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(pk_auto(Books::Id))
                    .col(integer(Books::AuthorId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_books_authors_author-id")
                            .from(Books::Table, Books::AuthorId)
                            .to(Authors::Table, Authors::Id),
                    )
                    .col(string(Books::Title))
                    .col(integer(Books::PublishedYear))
                    .col(string(Books::Genre))
                    .col(
                        timestamp_with_time_zone(Books::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Books::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    AuthorId,
    Title,
    PublishedYear,
    Genre,
    CreatedAt,
    UpdatedAt,
}
