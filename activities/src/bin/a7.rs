// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
use Color::*;

enum Color {
    red,
    blue,
    yellow,
}

fn main() {
    print_color(yellow);
}

fn print_color(color: Color) {
    match color {
        red => println!("Color is red."),
        blue => println!("Color is blue."),
        yellow => println!("Color is yellow."),
    }
}
