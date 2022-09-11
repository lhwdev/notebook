#[rocket::launch]
fn rocket() -> _ {
  rocket::build()
    // .attach(fairing)
}
