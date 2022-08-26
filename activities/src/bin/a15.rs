// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let mut tickets: Vec<Ticket> = Vec::new();
    tickets.push(Ticket::Backstage(40.6, "John".to_owned()));
    tickets.push(Ticket::Vip(80.0, "Mary".to_owned()));
    tickets.push(Ticket::Standard(20.0));

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => {
                println!("Backstage- Name: {:?}, price: {:?}", price, name)
            }
            Ticket::Vip(price, name) => {
                println!("Vip- Name: {:?}, price: {:?}", price, name)
            }
            Ticket::Standard(price) => {
                println!("Standard- Price: {:?}", price)
            }
        }
    }
}
