use features::{main, shared};
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

mod features;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![main::index, main::logs, main::ingest, shared::file],
        )
        .attach(Template::fairing())
        .attach(db::stage())
}
