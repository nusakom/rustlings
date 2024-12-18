#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // Wrap the vector inside an Arc to share it between threads.
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // Clone the Arc to get a reference-counted pointer for each thread.
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            // Filter numbers by offset and calculate the sum.
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
