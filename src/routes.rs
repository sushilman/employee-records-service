use super::repo;
use rocket::serde::json::Json;
use super::models::{ Department, DepartmentCreation, DepartmentCreationResponse, InsertableDepartment, Employee, EmployeeCreation, EmployeeCreationResponse, InsertableEmployee };
use rocket::http::Status;
use super::db_connection;

#[get("/")]
pub fn index() -> &'static str {
    "Employee Records API"
}

#[post("/departments", data = "<department>")]
pub fn create_department(department: Json<DepartmentCreation>) -> Result<Json<DepartmentCreationResponse>, Status> {
    let mut conn = db_connection::establish_connection();
    let department = InsertableDepartment::from_department_creation(department.into_inner());
    let result = repo::insert(department, &mut conn);
    let d: Department = result.unwrap();
    let response = DepartmentCreationResponse{
        link: format!("/departments/{}", d.department_id),
    };

    Ok(Json(response))
}

#[get("/departments")]
pub fn get_departments() -> Result<Json<Vec<Department>>, Status> {
    let mut conn = db_connection::establish_connection();
    let result = repo::get_departments(&mut conn);
    let departments: Vec<Department> = result.unwrap();

    Ok(Json(departments))
}

#[get("/departments/<department_id>")]
pub fn get_department_by_id(department_id: i32) -> Result<Json<Department>, Status> {
    let mut conn = db_connection::establish_connection();
    repo::get_department_by_id(department_id, &mut conn)
        .map(|d| Json(d))
        .map_err(|_err| Status::NotFound)
}

// TODO: handle error when department_id does not exist
#[post("/departments/<department_id>/employees", data = "<employee>")]
pub fn create_employees(department_id: i32, employee: Json<EmployeeCreation>) -> Result<Json<EmployeeCreationResponse>, Status> {
    let mut conn = db_connection::establish_connection();
    let employee = InsertableEmployee::from_employee_creation(employee.into_inner(), department_id);
    let result = repo::insert_employee(employee, &mut conn);
    result
    .map(|e| {
        let response = EmployeeCreationResponse{
            link: format!("/departments/{}/employee/{}", department_id, e.employee_id),
        };
    
        Json(response)
    })
    .map_err(|err| {
       match err {
            diesel::result::Error::DatabaseError(db_error_kind, _) => { 
                match db_error_kind {
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation => Status::NotFound,
                    _ => Status::InternalServerError
                }
            },
            _ => Status::InternalServerError
        }
    })
}
