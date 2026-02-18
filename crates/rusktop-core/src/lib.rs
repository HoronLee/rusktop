pub mod biz;
pub mod config;
pub mod data;
pub mod di;
pub mod migration;
pub mod proto;
pub mod server;
pub mod service;
pub mod ui_state;

pub use config::{AppConfig, DatabaseConfig, LogConfig, ServerConfig};
pub use ui_state::{ServiceStatus, WebServiceConfig};

use data::init_db;
use migration::Migrator;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;

pub async fn run(db_url: &str, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db(db_url).await?;

    db.get_schema_registry("rusktop_core::data::entity::*")
        .sync(&db)
        .await?;

    Migrator::up(&db, None).await?;

    let user_service = di::build_services(db);
    server::http::serve(user_service, addr).await?;
    Ok(())
}
