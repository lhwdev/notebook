use sea_orm_rocket::Database;
use database::db;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::Main::init())
        // .mount("/api", )
    // .attach(fairing)
}
