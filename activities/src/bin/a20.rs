// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn check_power_state(x: &str) {
        let state = x.trim().to_lowercase();
        let state_option = match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        };

        match state_option {
            Some(PowerState::Off) => println!("Turning Off..."),
            Some(PowerState::Sleep) => println!("Sleeping..."),
            Some(PowerState::Reboot) => println!("Rebooting..."),
            Some(PowerState::Shutdown) => println!("Shutting Down..."),
            Some(PowerState::Hibernate) => println!("Hibernating..."),
            None => println!("Not a valid power state."),
        }
    }
}

fn main() {
    use std::io;

    let mut buffer = String::new();
    println!("Enter command:");
    let input_result = io::stdin().read_line(&mut buffer);
    match input_result {
        Ok(_) => PowerState::check_power_state(&buffer),
        Err(e) => println!("error: {:?}", e),
    }
}
