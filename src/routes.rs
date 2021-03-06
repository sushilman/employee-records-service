use rocket::State;
use super::repo;
use rocket::serde::json::Json;
use super::models::{ Department, DepartmentCreation, DepartmentCreationResponse, InsertableDepartment, Employee, EmployeeCreation, EmployeeCreationResponse, InsertableEmployee };
use rocket::http::Status;
use super::db_connection;
use super::db_connection::PgPool;

#[get("/")]
pub fn index() -> &'static str {
    "Employee Records API"
}

#[post("/departments", data = "<department>")]
pub fn create_department(department: Json<DepartmentCreation>, pool: &State<PgPool>) -> Result<Json<DepartmentCreationResponse>, Status> {
    let mut conn = db_connection::pg_pool_handler(pool).unwrap();
    let department = InsertableDepartment::from_department_creation(department.into_inner());
    let result = repo::insert(department, &mut conn);
    let d: Department = result.unwrap();
    let response = DepartmentCreationResponse{
        link: format!("/departments/{}", d.department_id),
    };

    Ok(Json(response))
}

#[get("/departments")]
pub fn get_departments(pool: &State<PgPool>) -> Result<Json<Vec<Department>>, Status> {
    let mut conn = db_connection::pg_pool_handler(pool).unwrap();
    let result = repo::get_departments(&mut conn);
    let departments: Vec<Department> = result.unwrap();

    Ok(Json(departments))
}

#[get("/departments/<department_id>")]
pub fn get_department_by_id(department_id: i32, pool: &State<PgPool>) -> Result<Json<Department>, Status> {
    let mut conn = db_connection::pg_pool_handler(pool).unwrap();
    repo::get_department_by_id(department_id, &mut conn)
        .map(|d| Json(d))
        .map_err(|_err| Status::NotFound)
}

#[post("/departments/<department_id>/employees", data = "<employee>")]
pub fn create_employees(department_id: i32, employee: Json<EmployeeCreation>, pool: &State<PgPool>) -> Result<Json<EmployeeCreationResponse>, Status> {
    let mut conn = db_connection::pg_pool_handler(pool).unwrap();
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
#[get("/departments/<department_id>/employees")]
pub fn get_employees(department_id: i32, pool: &State<PgPool>) -> Result<Json<Vec<Employee>>, Status> {
    let mut conn = db_connection::pg_pool_handler(pool).unwrap();
    let result = repo::get_employees(department_id, &mut conn);
    let employees: Vec<Employee> = result.unwrap();

    Ok(Json(employees))
}

#[get("/departments/<department_id>/employees/<employee_id>")]
pub fn get_employee_by_id(department_id: i32, employee_id: i32, pool: &State<PgPool>) -> Result<Json<Employee>, Status> {
    let mut conn = db_connection::pg_pool_handler(pool).unwrap();
    repo::get_employee_by_id(department_id, employee_id, &mut conn)
        .map(|e| Json(e))
        .map_err(|_err| Status::NotFound)
}