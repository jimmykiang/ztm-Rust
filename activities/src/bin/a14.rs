// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(x: &str) {
    println!("{:?}", x);
}

fn main() {
    let mut people = Vec::new();
    people.push(Person {
        name: String::from("Jungle Georg"),
        fav_color: String::from("Green"),
        age: 9,
    });
    people.push(Person {
        name: String::from("Salina oldie"),
        fav_color: String::from("Red"),
        age: 99,
    });
    people.push(Person {
        name: String::from("Majonaise"),
        fav_color: String::from("Yellow"),
        age: 7,
    });

    for x in &people {
        match x.age {
            val if val <= 10 => {
                print(&x.name);
                print(&x.fav_color);
                print(&x.age.to_string());
                print("______________");
            }
            _ => (),
        }
    }
}
