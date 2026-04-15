use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 10;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];

    // Create 2 producer threads
    for id in 0..2 {
        let tx_clone = tx.clone();
        producer_handles.push(thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT);
        }));
    }

    // Create 3 consumer threads
    for id in 0..3 {
        let rx_clone = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Wait for producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    println!("All producers finished. Sending termination signals...");

    // Send termination signal for each consumer
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer function (NO rand)
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    for i in 0..item_count {
        // Deterministic "pseudo-data"
        let value = (id as i32) * 100 + i as i32;

        println!("Producer {} generated {}", id, value);

        tx.send(value).unwrap();

        thread::sleep(Duration::from_millis(200));
    }

    println!("Producer {} finished producing.", id);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        }

        println!("Consumer {} processing {}", id, value);

        thread::sleep(Duration::from_millis(300));
    }

    println!("Consumer {} exiting.", id);
}