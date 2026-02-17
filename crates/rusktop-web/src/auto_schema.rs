use rusktop_core::data::po::user;
use sea_orm::{ConnectionTrait, DbErr, DbConn, Schema, sea_query::TableCreateStatement};

pub async fn drop_and_create(db: &DbConn) -> Result<(), DbErr> {
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);
    
    let stmt: TableCreateStatement = schema.create_table_from_entity(user::Entity);
    
    db.execute(backend.build(&stmt)).await?;
    
    tracing::info!("✅ Auto-created tables from entities");
    Ok(())
}
