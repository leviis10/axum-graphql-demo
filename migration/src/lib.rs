pub use sea_orm_migration::prelude::*;

mod m20251007_165314_create_authors_table;
mod m20251007_165644_create_books_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251007_165314_create_authors_table::Migration),
            Box::new(m20251007_165644_create_books_table::Migration),
        ]
    }
}
