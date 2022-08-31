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

    fn menu_option(x: &str) -> Option<MainMenu> {
        match x {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            "3" => Some(MainMenu::DeleteBill),
            "4" => Some(MainMenu::UpdateBill),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

use std::collections::HashMap;

pub struct Bills {
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

    fn get_all_bill(&self) -> Vec<&Bill> {
        self.bills.values().collect()
    }

    fn contains_bill(&self, name: &str) -> bool {
        self.bills.contains_key(name)
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.bills.get_mut(name) {
            Some(x) => {
                x.amount = amount;
                true
            }
            None => false,
        }
    }

    fn remove_bill(&mut self, name: &str) -> bool {
        self.bills.remove(name).is_some()
    }
}

mod menu {
    use crate::{Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        loop {
            println!("Enter name of the bill:");
            let bill_name = match get_input() {
                Some(x) => x,
                None => continue,
            };

            let bill_amount = match get_bill_amount_input() {
                Some(x) => x,
                None => continue,
            };

            let bill = Bill {
                name: bill_name,
                amount: bill_amount,
            };

            // Clone to not make the bill struct get lost in the move to println!.
            println!("Bill: {:?}", bill.clone());

            bills.add_bill(bill);
            println!("Bill inserted.");
            break;
        }
    }

    pub fn get_bills(bills: &Bills) {
        for x in bills.get_all_bill() {
            println!("{:?}", x);
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        loop {
            println!("Enter bill name to be updated:");

            let name = match get_input() {
                Some(x) => x,
                None => continue,
            };

            if !bills.contains_bill(&name) {
                println!("Not found.");
                break;
            }

            let amount = match get_bill_amount_input() {
                Some(x) => x,
                None => continue,
            };

            if bills.update(&name, amount) {
                println!("Entry updated");
            } else {
                println!("Not found.");
            }
            break;
        }
    }

    pub fn delete_bill(bills: &mut Bills) {
        loop {
            println!("Enter bill name to be removed:");
            let bill_name = match get_input() {
                Some(x) => x,
                None => continue,
            };

            if bills.remove_bill(&bill_name) {
                println!("Entry deleted.");
            } else {
                println!("Not Found.");
            }
            break;
        }
    }

    pub fn get_input() -> Option<String> {
        use std::io;
        let mut buffer = String::new();
        loop {
            let input_result = io::stdin().read_line(&mut buffer);
            match input_result {
                Ok(_) => {
                    let input_string = buffer.trim().to_owned();
                    return if input_string == "" {
                        None
                    } else {
                        Some(input_string)
                    };
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    fn get_bill_amount_input() -> Option<f64> {
        loop {
            println!("Enter amount:");
            let bill_amount_input = match get_input() {
                Some(x) => x,
                None => continue,
            };

            use std::num::ParseFloatError;
            let parsed_result: Result<f64, ParseFloatError> = bill_amount_input.parse();
            match parsed_result {
                Ok(x) => return Some(x),
                Err(e) => println!("Bill amount error: {:?}", e),
            }
        }
    }
}

fn main_menu_loop() {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let menu_input = menu::get_input();
        let menu_option = match menu_input {
            Some(x) => x,
            None => break,
        };
        match MainMenu::menu_option(&menu_option) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::get_bills(&bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            Some(MainMenu::DeleteBill) => menu::delete_bill(&mut bills),
            None => break,
        }
    }
}

fn main() {
    main_menu_loop();
}
