use async_trait::async_trait;
use sea_orm::DbErr;
use std::sync::Arc;

use crate::data::UserRepository;
use crate::data::entity::user;

#[async_trait]
pub trait UserUseCase: Send + Sync {
    async fn create_user(&self, name: String, age: i32) -> Result<user::Model, DbErr>;
    async fn get_user(&self, id: i32) -> Result<Option<user::Model>, DbErr>;
    async fn list_users(&self) -> Result<Vec<user::Model>, DbErr>;
    async fn update_user(&self, id: i32, name: String, age: i32) -> Result<user::Model, DbErr>;
    async fn delete_user(&self, id: i32) -> Result<(), DbErr>;
}

pub struct UserUseCaseImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserUseCaseImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn create_user(&self, name: String, age: i32) -> Result<user::Model, DbErr> {
        self.repo.create(name, age).await
    }

    async fn get_user(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        self.repo.get_by_id(id).await
    }

    async fn list_users(&self) -> Result<Vec<user::Model>, DbErr> {
        self.repo.list().await
    }

    async fn update_user(&self, id: i32, name: String, age: i32) -> Result<user::Model, DbErr> {
        self.repo.update(id, name, age).await
    }

    async fn delete_user(&self, id: i32) -> Result<(), DbErr> {
        self.repo.delete(id).await
    }
}
