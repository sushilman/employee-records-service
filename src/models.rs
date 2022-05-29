use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Department {
    pub department_id: usize,
    pub name: String,
    pub created_at: String,
    pub modified_at: String,
}
