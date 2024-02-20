pub mod models;

use rocket::fairing::AdHoc;
use rocket_db_pools::{sqlx, Database};

/// Create a Rocket fairing to bootstrap the DB.
#[derive(Database)]
#[database("logs")]
pub struct Db(sqlx::SqlitePool);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx stage", |rocket| async {
        rocket.attach(Db::init())
        // Add more if needed
    })
}
