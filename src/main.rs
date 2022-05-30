#[macro_use]extern crate rocket;
#[macro_use]extern crate diesel;

mod routes;
mod models;
mod schema;
mod repo;
mod db_connection;

#[launch]
fn rocket() -> _ {
    //let connection = db_connection::establish_connection();
    
    rocket::build()
        .mount("/", routes![
            routes::index,
            routes::get_departments, // GET /departments
            routes::get_department_by_id, // GET /departments/<department_id>
            routes::create_department, // POST /departments

        ])
}
