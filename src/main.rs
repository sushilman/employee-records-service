#[macro_use] extern crate rocket;

mod routes;
mod models;

#[launch]
fn app() -> _ {
    rocket::build()
        .mount("/", routes![
            routes::index,
            routes::get_departments, // GET /departments
        ])
}
