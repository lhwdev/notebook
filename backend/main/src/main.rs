use database::db;
use sea_orm_rocket::Database;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().attach(db::Main::init())
    // .mount("/api", )
    // .attach(fairing)
}
