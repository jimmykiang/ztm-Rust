// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

enum MainMenu {
    AddBill,
    ViewBill,
    DeleteBill,
    UpdateBill,
}

impl MainMenu {
    fn show() {
        println!("");
        println!("Select from the options below:");
        println!("1- Add Bill.");
        println!("2- View Bill.");
        println!("3- Delete Bill.");
        println!("4- Update Bill.");
        println!("");
    }
}

struct Bill {
    name: String,
    amount: f64,
}

use std::collections::HashMap;

struct Bills {
    bills: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            bills: HashMap::new(),
        }
    }

    fn add_bill(&mut self, bill: Bill) {
        // clone the name field from bill, otherwise the name field would be
        // moved out from the bill.name field and into the key of the hashMap.
        self.bills.insert(bill.name.clone(), bill);
    }
}

fn main_menu_loop() {
    MainMenu::show();
}

fn main() {
    main_menu_loop();
}
