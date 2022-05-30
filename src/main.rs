#[macro_use] extern crate rocket;

mod routes;
mod models;

#[launch]
fn app() -> _ {
    rocket::build()
        .mount("/", routes![
            routes::index,
            routes::get_departments, // GET /departments
            routes::get_department_by_id, // GET /departments/<department_id>
            routes::create_department, // POST /departments

        ])
}
