use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // Wrap the JobStatus struct in a Mutex, then wrap the Mutex in an Arc for shared ownership.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock the Mutex to safely update the shared value.
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the value of JobStatus.jobs_done after all threads are done.
    let status = status.lock().unwrap(); // Lock to safely read the value.
    println!("Jobs done: {}", status.jobs_done);
}
