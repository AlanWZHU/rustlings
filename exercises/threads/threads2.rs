// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            // thread::sleep(Duration::from_millis(250));
            thread::sleep(Duration::from_secs(1));
            // TODO: You must take an action before you update a shared value
            let mut jobs_completed = status_shared.jobs_completed.lock().unwrap();
            *jobs_completed += 1;
            println!("jobs completed in thread  {}",&jobs_completed);
        });
        handles.push(handle);
    }
    for handle in handles {
        let is_finished=handle.is_finished();
        println!(" is_finished{}",is_finished);
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        // TODO 为何输出结果是10 而不是1 到10 的无序打印 
        // 在本机上 的确是偶然现象 在循环join的过程中 子线程在大部分一两次后都执行完毕 
        println!("jobs completed {}",status.jobs_completed.lock().unwrap());
    }
    // println!("jobs completed after handle {}",status.jobs_completed.lock().unwrap()) 
}
