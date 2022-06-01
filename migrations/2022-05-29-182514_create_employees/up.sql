-- Employee table
CREATE TABLE employees (
  employee_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  department_id INT NOT NULL, 
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  active BOOLEAN NOT NULL DEFAULT 't',
  CONSTRAINT fk_department
    FOREIGN KEY(department_id)
    REFERENCES departments(department_id)
);