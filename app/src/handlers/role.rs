use crate::entities::roles;
use crate::errors::Result;
use crate::repositories;
use sea_orm::DatabaseConnection;

pub async fn find_by_name(
    db: &DatabaseConnection,
    roles: Vec<String>,
) -> Result<Vec<roles::Model>> {
    repositories::role::find_by_name(db, roles).await
}
