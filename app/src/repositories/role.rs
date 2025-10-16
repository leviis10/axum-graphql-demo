use crate::entities::prelude::Roles;
use crate::entities::roles;
use crate::errors::Result;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait};

pub async fn find_by_name(
    connection: &impl ConnectionTrait,
    roles: Vec<String>,
) -> Result<Vec<roles::Model>> {
    let found_role = Roles::find()
        .filter(roles::Column::Name.is_in(roles))
        .all(connection)
        .await?;

    Ok(found_role)
}
