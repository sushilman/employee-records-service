use rocket::serde::json::Json;
use super::models::{ Department, InsertableDepartment };
use rocket::http::Status;

#[get("/")]
pub fn index() -> &'static str {
    "Employee Records API"
}

#[post("/departments", data = "<department>")]
pub fn create_department(department: Json<InsertableDepartment<'_>>) -> Status {
    department.name;
    Status::Created
}

#[get("/departments")]
pub fn get_departments() -> Result<Json<Vec<Department>>, Status> {
    // TODO: implement
    
    // This is a mock implementation
    let departments = vec![
        Department{
            department_id: 1,
            name: String::from("Sales"),
            created_at: String::from("2022-05-29T20:16:38+00:00"),
            modified_at: String::from("2022-05-29T20:16:38+00:00"),
        },
        Department{
            department_id: 2,
            name: String::from("Engineering"),
            created_at: String::from("2022-05-29T20:16:38+00:00"),
            modified_at: String::from("2022-05-29T20:16:38+00:00"),
        },
    ];

    Ok(Json(departments))
}

#[get("/departments/<department_id>")]
pub fn get_department_by_id(department_id: usize) -> Result<Json<Department>, Status> {
    // TODO: implement
    
    // This is a mock implementation
    let department = Department{
        department_id: department_id,
        name: String::from("Sales"),
        created_at: String::from("2022-05-29T20:16:38+00:00"),
        modified_at: String::from("2022-05-29T20:16:38+00:00"),
    };

    Ok(Json(department))
}