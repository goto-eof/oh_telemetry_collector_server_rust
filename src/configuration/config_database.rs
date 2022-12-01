use log::debug;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, ConnectionTrait, Statement};
use sea_orm::{Database, DbConn};

use crate::{DB_POOL, SETTINGS};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let opt = ConnectOptions::new(SETTINGS.db_uri.to_owned());
    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;
    debug!("DB Connection OK: {}", SETTINGS.db_uri);
    Ok(db)
}

pub async fn init_db() {
    debug!("Checking DB connection...");
    let db = DB_POOL.get().await;
    let result = db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT 1 from seaql_migrations limit 1;".to_owned(),
        ))
        .await;
    if result.is_err() {
        debug!("[DB RESULT] Connection to [DB FAILED]: {:?}", result.err());
    } else {
        debug!("[DB RESULT] DB Connection [OK]")
    }
}
