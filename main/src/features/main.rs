use db::{models::log::Log, Db};
use payloads::payload::Payload;
use rocket::{get, post};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde_json;

#[get("/")]
pub async fn index() -> Template {
    Template::render("shared/layout", context! {})
}

#[get("/logs")]
pub async fn logs(mut db: Connection<Db>) -> Template {
    let logs: Result<Vec<Log>, _> =
        sqlx::query_as::<_, Log>(r"SELECT id, date, data FROM logs ORDER BY date DESC")
            .fetch_all(&mut **db)
            .await;

    log::debug!("{:?}", logs);
    Template::render("main/logentry", context! { logs: logs.unwrap_or_default() })
}

#[get("/new-logs/<date>")]
pub async fn new_logs<'a>(mut db: Connection<Db>, date: &'a str) -> Template {
    let logs: Result<Vec<Log>, _> =
        sqlx::query_as::<_, Log>(format!(r"SELECT * FROM logs WHERE DATETIME(date) > DATETIME('{}') ORDER BY date DESC", date).as_str())
            .fetch_all(&mut **db)
            .await;

    log::debug!("{:?}", logs);
    Template::render("main/newlogs", context! { logs: logs.unwrap_or_default()})
}

#[post("/ingest", format = "application/json", data = "<payload>")]
pub async fn ingest(mut db: Connection<Db>, payload: String) {
    let payload_value: serde_json::Value = serde_json::from_str(&payload[..]).unwrap();
    let p: Payload = serde_json::from_str(&payload[..]).unwrap();
    let result = sqlx::query(r"INSERT INTO logs (data) VALUES (?)")
        .bind(&payload_value["payload_data"])
        .execute(&mut **db)
        .await;

    match result {
        Err(e) => {
            println!("Error inserting payload {:?}: {}", p, e);
        }
        Ok(res) => {
            println!("Inserted. Number of rows affected: {}", res.rows_affected());
        }
    }
}
