table! {
    departments (department_id) {
        department_id -> Int4,
        name -> Varchar,
        created_at -> Date,
        updated_at -> Date,
    }
}

table! {
    employees (employee_id) {
        employee_id -> Int4,
        name -> Varchar,
        email -> Varchar,
        department_id -> Nullable<Int4>,
        created_at -> Date,
        updated_at -> Date,
        active -> Bool,
    }
}

joinable!(employees -> departments (department_id));

allow_tables_to_appear_in_same_query!(
    departments,
    employees,
);
