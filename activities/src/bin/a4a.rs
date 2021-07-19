// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let x: bool = false;

    match x {
        true => println!("it was true"),
        false => println!("it was false"),
    }
}
