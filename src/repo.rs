
use diesel::prelude::*;
use super::models::{ InsertableDepartment, Department, InsertableEmployee, Employee };
use super::schema::{ departments, employees };

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

pub fn insert_employee(employee: InsertableEmployee, connection: &mut PgConnection) -> QueryResult<Employee> {
    diesel::insert_into(employees::table)
        .values(employee)
        .get_result(connection)
}
