// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(800));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(500));
    "!"
}

fn main() {
    use std::thread;
    let msg_1 = thread::spawn(move || msg_hello());
    let msg_2 = thread::spawn(move || msg_thread());
    let msg_3 = thread::spawn(move || msg_excited());

    let msg_1 = msg_1.join().expect("Error join msg_1");
    let msg_2 = msg_2.join().expect("Error join msg_2");
    let msg_3 = msg_3.join().expect("Error join msg_3");

    println!("{:?}{:?}{:?}", msg_1, msg_2, msg_3);
}
