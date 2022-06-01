table! {
    departments (department_id) {
        department_id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    employees (employee_id) {
        employee_id -> Int4,
        name -> Varchar,
        email -> Varchar,
        department_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        active -> Bool,
    }
}

joinable!(employees -> departments (department_id));

allow_tables_to_appear_in_same_query!(
    departments,
    employees,
);
