#[macro_use] extern crate rocket;

#[launch]
fn app() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Employee Records API"
}
