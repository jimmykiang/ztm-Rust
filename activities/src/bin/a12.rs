// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Dimensions {
    x: f64,
    y: f64,
    z: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("X: {:?}", self.x);
        println!("Y: {:?}", self.y);
        println!("Z: {:?}", self.z);
    }
}

enum Color {
    Blue,
    Yellow,
}

impl Color {
    fn print(&self) {
        match self {
            self::Color::Blue => {
                println!("Color Blue.");
            }
            self::Color::Yellow => {
                println!("Color Yellow.");
            }
        }
    }
}

struct ShippingBox {
    dimension: Dimensions,
    color: Color,
}

impl ShippingBox {
    fn new(dimension: Dimensions, color: Color) -> Self {
        Self {
            color: color,
            dimension: dimension,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimension.print();
    }
}

fn main() {
    let dimension_1: Dimensions = Dimensions {
        x: 3.0,
        y: 4.5,
        z: 5.9,
    };
    let box_1 = ShippingBox::new(dimension_1, Color::Yellow);

    box_1.print();
}
