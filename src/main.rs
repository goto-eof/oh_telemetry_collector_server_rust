use async_once::AsyncOnce;
use sea_orm::DbConn;

use crate::{
    configuration::{
        config_database::{self, init_db},
        config_loader::Settings,
        config_logging::init_logging,
    },
    route::routes_util::init_routes,
};
mod configuration;
mod controller;
mod dao;
mod entity;
mod route;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: Settings = Settings::init_configuration().unwrap();
    static ref DB_POOL: AsyncOnce<DbConn> = AsyncOnce::new(async {
        let db = config_database::establish_connection().await;
        db.unwrap()
    });
}

#[tokio::main]
async fn main() {
    init_logging();
    init_db().await;

    println!("================================");
    println!("server started on port [8013] :)");
    println!("================================");

    warp::serve(init_routes().await)
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}
