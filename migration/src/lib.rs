pub use sea_orm_migration::prelude::*;

mod m20251007_165314_create_authors_table;
mod m20251007_165644_create_books_table;
mod m20251014_131151_create_users_table;
mod m20251014_141128_create_roles_table;
mod m20251014_142953_create_join_table_users_roles;
mod m20251014_152859_create_roles_table_seeder;
mod m20251016_020852_create_refresh_tokens_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251007_165314_create_authors_table::Migration),
            Box::new(m20251007_165644_create_books_table::Migration),
            Box::new(m20251014_131151_create_users_table::Migration),
            Box::new(m20251014_141128_create_roles_table::Migration),
            Box::new(m20251014_142953_create_join_table_users_roles::Migration),
            Box::new(m20251014_152859_create_roles_table_seeder::Migration),
            Box::new(m20251016_020852_create_refresh_tokens_table::Migration),
        ]
    }
}
