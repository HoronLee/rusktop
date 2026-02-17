use std::sync::Arc;

use crate::biz::{UserUseCase, UserUseCaseImpl};
use crate::data::{Data, DataImpl, DataParameters, UserRepository, UserRepositoryImpl};

use crate::service::user_service::UserServiceImpl;

pub fn build_services(db: sea_orm::DatabaseConnection) -> Arc<UserServiceImpl> {
    let data: Arc<dyn Data> = Arc::new(DataImpl::new(DataParameters { db }));
    let repo: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(data));
    let use_case: Arc<dyn UserUseCase> = Arc::new(UserUseCaseImpl::new(repo));
    Arc::new(UserServiceImpl::new(use_case))
}
