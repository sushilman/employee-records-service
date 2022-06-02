#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use crate::models::ApiError;
use rocket::serde::json::Json;
use rocket::Request;

mod routes;
mod models;
mod schema;
mod repo;
mod db_connection;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(db_connection::establish_connection())
        .register("/", catchers![not_found, internal_server_error])
        .mount("/", routes![
            routes::index,
            routes::get_departments, // GET /departments
            routes::get_department_by_id, // GET /departments/<department_id>
            routes::create_department, // POST /departments
            routes::create_employees,  // POST /departments/<department_id>/employees
            routes::get_employees, // GET /departments/<department_id>/employees
            routes::get_employee_by_id, // GET /departments/<department_id>/employees/<employee_id>
        ])
}

#[catch(404)]
fn not_found(_req: &Request) -> Json<ApiError> { 
    Json(ApiError{
        status: 404,
        message: String::from("The requested resource was not found"),
    })
}

#[catch(500)]
fn internal_server_error(_req: &Request) -> Json<ApiError> { 
    Json(ApiError{
        status: 500,
        message: String::from("Something went wrong. Try again later."),
    })
}