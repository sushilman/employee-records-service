use rocket::serde::{Deserialize, Serialize};
use chrono::{ DateTime, Utc };
use super::schema::departments;

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