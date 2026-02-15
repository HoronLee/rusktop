use async_trait::async_trait;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::proto::rusktop::service::v1::user_service_server::UserService;
use crate::proto::user::service::v1::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest,
    GetUserResponse, ListUsersRequest, ListUsersResponse, UpdateUserRequest, UpdateUserResponse,
    User,
};
use rusktop_core::biz::UserUseCase;

pub struct UserServiceImpl {
    use_case: Arc<dyn UserUseCase>,
}

impl UserServiceImpl {
    pub fn new(use_case: Arc<dyn UserUseCase>) -> Self {
        Self { use_case }
    }
}

fn model_to_proto(m: &rusktop_core::entity::user::Model) -> User {
    User {
        id: m.id,
        name: m.name.clone(),
        age: m.age,
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let user = self
            .use_case
            .create_user(req.name, req.age)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateUserResponse {
            user: Some(model_to_proto(&user)),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        let user = self
            .use_case
            .get_user(req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or(Status::not_found("User not found"))?;

        Ok(Response::new(GetUserResponse {
            user: Some(model_to_proto(&user)),
        }))
    }

    async fn list_users(
        &self,
        _request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        let users = self
            .use_case
            .list_users()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListUsersResponse {
            users: users.iter().map(model_to_proto).collect(),
        }))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let req = request.into_inner();
        let user = self
            .use_case
            .update_user(req.id, req.name, req.age)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateUserResponse {
            user: Some(model_to_proto(&user)),
        }))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let req = request.into_inner();
        self.use_case
            .delete_user(req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteUserResponse { success: true }))
    }
}
