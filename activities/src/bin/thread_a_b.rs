use crossbeam::channel;
use std::thread;

fn main() {

    let (tx_a, rx_a):(crossbeam::Sender<&str>, crossbeam::Receiver<&str>) = channel::unbounded();
    let (tx_b, rx_b):(crossbeam::Sender<&str>, crossbeam::Receiver<&str>) = channel::unbounded();

    let handle_a = thread::spawn(move || {

        tx_a.send("Sent from thread A.").unwrap();
        for msg in rx_b {

            println!("Received in A: {:?}", msg);
            break;
        }
    });

    let handle_b = thread::spawn(move || {

        for msg in rx_a {
            println!("Received in B: {:?}", msg);
            tx_b.send("Sent from thread B.").unwrap();
        }
    });

    handle_a.join().unwrap();
    handle_b.join().unwrap();
    println!("Main thread: Exiting.")
}