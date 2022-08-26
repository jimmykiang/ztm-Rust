// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Maintenance,
    Marketing,
    Manager,
    Line_supervisor,
    Kitchen_staff,
    Assembly_tech,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn verify_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Terminated".to_owned()),
        _ => (),
    }
    match employee.position {
        Position::Manager => Ok(()),
        _ => Err("Current position: cannot obtain access.".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let access = verify_access(employee)?;
    println!("Access granted.");
    Ok(())
}

fn main() {
    let line_supervisor = Employee {
        position: Position::Line_supervisor,
        status: Status::Active,
    };

    match print_access(&line_supervisor) {
        Err(e) => println!("Access status: {:?}", e),
        _ => (),
    }
}
