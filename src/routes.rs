use rocket::serde::json::Json;
use super::models::Department;

#[get("/")]
pub fn index() -> &'static str {
    "Employee Records API"
}

#[get("/departments")]
pub fn get_departments() -> Json<Vec<Department>> {
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

    Json(departments)
}
