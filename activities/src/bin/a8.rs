// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Lemon,
    Orange,
    Apple,
}

struct Drink {
    flavor: Flavor,
    volume_milliliter: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Lemon => {
            println!("Flavor Lemon, volume: {:?}", drink.volume_milliliter);
        }
        Flavor::Orange => {
            println!("Flavor Orange, volume: {:?}", drink.volume_milliliter);
        }
        Flavor::Apple => {
            println!("Flavor Apple, volume: {:?}", drink.volume_milliliter);
        }
    }
}

fn main() {
    let orange_juice: Drink = Drink {
        flavor: Flavor::Orange,
        volume_milliliter: 400.0,
    };

    print_drink(orange_juice);

    let apple_juice: Drink = Drink {
        flavor: Flavor::Apple,
        volume_milliliter: 30.66,
    };

    print_drink(apple_juice);
}
