// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id: i32,
}

fn display_quantity(x: &Grocery) {
    println!("Quantity: {:?}", x.quantity);
}

fn display_id(x: &Grocery) {
    println!("Id: {:?}", x.id);
}

fn main() {
    let item = &Grocery {
        quantity: 5,
        id: 40,
    };

    display_quantity(item);
    display_id(item);
}
