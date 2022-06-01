use rocket::serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };
use super::schema::{ departments, employees };

#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
//#[table_name = "departments"]
pub struct Department {
    pub department_id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DepartmentCreation{
    pub name: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DepartmentCreationResponse{
    pub link: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "departments"]
pub struct InsertableDepartment {
    pub name: String,
}

impl InsertableDepartment {
    pub fn from_department_creation(d: DepartmentCreation) -> InsertableDepartment {
        InsertableDepartment {
            name: d.name,
        }
    }
}

/*
    employees (employee_id) {
        employee_id -> Int4,
        name -> Varchar,
        email -> Varchar,
        department_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        active -> Bool,
    }
*/

#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
//#[table_name = "employees"]
pub struct Employee {
    pub employee_id: i32,
    pub name: String,
    pub email: String,
    pub department_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EmployeeCreation{
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EmployeeCreationResponse{
    pub link: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "employees"]
pub struct InsertableEmployee {
    pub name: String,
    pub email: String,
    pub department_id: i32,
}

impl InsertableEmployee {
    pub fn from_employee_creation(e: EmployeeCreation, dept_id: i32) -> InsertableEmployee {
        InsertableEmployee {
            name: e.name,
            email: e.email,
            department_id: dept_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub message: String, 
}

