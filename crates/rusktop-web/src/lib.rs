pub mod auto_schema;
pub mod di;
pub mod migration;
pub mod proto;
pub mod server;
pub mod service;

use migration::Migrator;
use rusktop_core::data::init_db;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;

pub async fn run(
    db_url: &str,
    addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db(db_url).await?;
    
    Migrator::up(&db, None).await?;
    
    let user_service = di::build_services(db);
    server::http::serve(user_service, addr).await?;
    Ok(())
}