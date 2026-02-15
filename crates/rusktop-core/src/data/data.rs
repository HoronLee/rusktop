use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub struct DataParameters {
    pub db: DatabaseConnection,
}

pub trait Data: Send + Sync {
    fn db(&self) -> &DatabaseConnection;
}

pub struct DataImpl {
    db: DatabaseConnection,
}

impl DataImpl {
    pub fn new(params: DataParameters) -> Self {
        Self { db: params.db }
    }
}

impl Data for DataImpl {
    fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(10).min_connections(1);
    Database::connect(opt).await
}
