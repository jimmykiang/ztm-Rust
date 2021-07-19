// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:§
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let (x, y) = coordinate();

    match y {
        val if val > 5 => println!("Greater than 5."),
        val if val < 5 => println!("Less than 5."),
        val if val == 5 => println!("Equal to 5."),
        _ => println!("Anything else."),
    }
}

fn coordinate() -> (i32, i32) {
    (4, 5)
}
