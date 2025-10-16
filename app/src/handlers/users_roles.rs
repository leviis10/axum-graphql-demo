use crate::dtos::users_roles::AssignRoleRequest;
use crate::entities::users_roles;
use crate::errors::Result;
use crate::repositories;
use sea_orm::{ActiveValue, DatabaseTransaction};

pub async fn assign_role(
    txn: &DatabaseTransaction,
    request: AssignRoleRequest,
) -> Result<users_roles::Model> {
    let model = users_roles::ActiveModel {
        user_id: ActiveValue::Set(request.user_id),
        role_id: ActiveValue::Set(request.role_id),
    };

    repositories::users_roles::insert(txn, model).await
}
