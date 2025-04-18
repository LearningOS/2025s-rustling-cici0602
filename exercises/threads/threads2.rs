// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: AtomicU32,
}

fn main() {
    let status = Arc::new(JobStatus { 
        jobs_completed: AtomicU32::new(0)
    });
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        
    }
    println!("jobs completed {}",status.jobs_completed.load(Ordering::SeqCst));
}
