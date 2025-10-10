use crate::entities::authors;
use crate::entities::prelude::Authors;
use crate::errors::Result;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, ModelTrait, TryIntoModel};

pub async fn save(
    connection: &impl ConnectionTrait,
    model: authors::ActiveModel,
) -> Result<authors::Model> {
    let new_author = model.save(connection).await?.try_into_model()?;

    Ok(new_author)
}

pub async fn get_by_id(
    connection: &impl ConnectionTrait,
    id: i32,
) -> Result<Option<authors::Model>> {
    let found_author = Authors::find_by_id(id).one(connection).await?;

    Ok(found_author)
}

pub async fn find_all(connection: &impl ConnectionTrait) -> Result<Vec<authors::Model>> {
    let found_authors = Authors::find().all(connection).await?;

    Ok(found_authors)
}

pub async fn delete(connection: &impl ConnectionTrait, model: authors::Model) -> Result<()> {
    model.delete(connection).await?;

    Ok(())
}
