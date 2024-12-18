use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    // Spawn the first thread with access to the first_half.
    let tx_clone = Arc::clone(&tx);
    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");
            tx_clone.lock().unwrap().send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // Spawn the second thread with access to the second_half.
    let tx_clone = Arc::clone(&tx);
    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            tx_clone.lock().unwrap().send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx)); // Wrap `tx` in `Arc<Mutex<_>>`
    let queue = Queue::new();

    send_tx(queue, tx);

    let mut received = Vec::with_capacity(10);
    for value in rx {
        received.push(value);
    }

    received.sort();
    assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let tx = Arc::new(Mutex::new(tx)); // Wrap `tx` in `Arc<Mutex<_>>`
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
