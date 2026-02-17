use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, Set};
use std::sync::Arc;

use super::po::user;
use super::data::Data;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, name: String, age: i32) -> Result<user::Model, DbErr>;
    async fn get_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr>;
    async fn list(&self) -> Result<Vec<user::Model>, DbErr>;
    async fn update(&self, id: i32, name: String, age: i32) -> Result<user::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

pub struct UserRepositoryImpl {
    data: Arc<dyn Data>,
}

impl UserRepositoryImpl {
    pub fn new(data: Arc<dyn Data>) -> Self {
        Self { data }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, name: String, age: i32) -> Result<user::Model, DbErr> {
        let user = user::ActiveModel {
            name: Set(name),
            age: Set(age),
            ..Default::default()
        };
        user.insert(self.data.db()).await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(self.data.db()).await
    }

    async fn list(&self) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(self.data.db()).await
    }

    async fn update(&self, id: i32, name: String, age: i32) -> Result<user::Model, DbErr> {
        let user = user::Entity::find_by_id(id)
            .one(self.data.db())
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        let mut active: user::ActiveModel = user.into();
        active.name = Set(name);
        active.age = Set(age);
        active.update(self.data.db()).await
    }

    async fn delete(&self, id: i32) -> Result<(), DbErr> {
        user::Entity::delete_by_id(id)
            .exec(self.data.db())
            .await?;
        Ok(())
    }
}
