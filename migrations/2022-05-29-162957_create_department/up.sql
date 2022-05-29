-- Departments table
CREATE TABLE departments (
  department_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created_at DATE NOT NULL DEFAULT NOW(),
  updated_at DATE NOT NULL DEFAULT NOW()
);