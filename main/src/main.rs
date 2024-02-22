use env_logger;
use features::{main, shared};
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

mod features;

#[launch]
fn rocket() -> _ {
    env_logger::init();
    rocket::build()
        .mount(
            "/",
            routes![
                main::index,
                main::logs,
                main::ingest,
                main::new_logs,
                shared::file,
            ],
        )
        .attach(Template::fairing())
        .attach(db::stage())
}
