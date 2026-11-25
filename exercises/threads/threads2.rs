// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Arc alone shares ownership across threads but gives no interior
    // mutability; Mutex is what makes the counter writable (and serialises the
    // increments so no update is lost).
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Take the lock before touching the shared value. The guard is
            // dropped at the end of this statement, releasing the lock.
            let mut guard = status_shared.lock().unwrap();
            guard.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // Joining all the handles is what guarantees the count only grows to 10
        // after every worker has finished; printing before joining would show a
        // partially completed number.
        println!(
            "jobs completed {}",
            status.lock().unwrap().jobs_completed
        );
    }
}
