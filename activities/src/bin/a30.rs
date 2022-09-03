// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

#[derive(Debug)]
struct Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    body: T,
    color: U,
}

impl<T, U> Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    fn new(body: T, color: U) -> Self {
        Self { body, color }
    }
}

#[derive(Debug)]
struct Car {}

impl Body for Car {}

struct Truck {}

impl Body for Truck {}

struct Red {}

impl Color for Red {}

#[derive(Debug)]
struct White {}

impl Color for White {}

fn main() {
    let white_color = White {};
    let car = Car {};

    let white_car = Vehicle::new(car, white_color);

    println!("{:?}", white_car);
}
