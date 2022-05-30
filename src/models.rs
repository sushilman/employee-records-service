use rocket::request::FromRequest;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Department {
    pub department_id: usize,
    pub name: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InsertableDepartment<'a> {
    pub name: &'a str,
}

impl InsertableDepartment<'_> {

}