
use diesel::prelude::*;
use super::models::{ InsertableDepartment, Department };
use super::schema::departments;

pub fn insert(department: InsertableDepartment, connection: &mut PgConnection) -> QueryResult<Department> {
    diesel::insert_into(departments::table)
        .values(department)
        .get_result(connection)
}

pub fn get_departments(connection: &mut PgConnection) -> QueryResult<Vec<Department>> {
    departments::table.load(connection)
}

pub fn get_department_by_id(department_id: i32, connection: &mut PgConnection) -> QueryResult<Department> {
    departments::table.find(department_id).get_result(connection)
}
