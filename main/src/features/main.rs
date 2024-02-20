use db::{models::log::Log, Db};
use payloads::payload::{Payload, PayloadData};
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
    let logs: Result<Vec<Log>, _> = sqlx::query_as::<_, Log>(
        r"SELECT id, date, type as log_type, data FROM logs ORDER BY date DESC",
    )
    .fetch_all(&mut **db)
    .await;
    Template::render("main/logentry", context! { logs: logs.unwrap_or_default() })
}

#[get("/new-logs")]
pub async fn new_logs(mut db: Connection<Db>) -> Template {
    let logs: Result<Vec<Log>, _> =
        sqlx::query_as::<_, Log>(r"SELECT * FROM logs WHERE date >= now() ORDER BY date DESC")
            .fetch_all(&mut **db)
            .await;

    println!("{:?}", logs);

    Template::render("main/logentry", context! { logs: logs.unwrap_or_default()})
}

#[post("/ingest", format = "application/json", data = "<payload>")]
pub async fn ingest(mut db: Connection<Db>, payload: String) {
    let payload_value: serde_json::Value = serde_json::from_str(&payload[..]).unwrap();
    let p: Payload = serde_json::from_str(&payload[..]).unwrap();
    let result = sqlx::query(r"INSERT INTO logs (type, data) VALUES (?, ?)")
        .bind(&payload_value["type"])
        .bind(&payload_value["data"])
        .execute(&mut **db)
        .await;

    match result {
        Err(e) => {
            println!("Error inserting payload {:?}", p);
        }
        Ok(res) => {
            println!("Inserted. Number of rows affected: {}", res.rows_affected());
        }
    }
}