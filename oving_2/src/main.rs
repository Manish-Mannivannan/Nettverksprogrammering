pub mod thread_pool;

use std::{thread, time::Duration};

use crate::thread_pool::Pool;

fn task() {
    println!("\tHello, from task, thread_id: {:?}", thread::current().id());
    thread::sleep(Duration::from_secs(2));
}

fn main() {
    run_workers();
    event_loop();
}

fn run_workers(){
    let mut workers = Pool::new(2);
    workers.start();
    for i in 0..5 {
        thread::sleep(std::time::Duration::from_millis(200));
        println!("Posting task nr: {}", i);
        workers.post(task);
    }
    workers.post_timeout(task, Duration::from_micros(3000));
    workers.stop_and_finish();
}

fn event_loop(){
    let mut eventloop = Pool::new(1);

    for i in 0..5 {
        thread::sleep(std::time::Duration::from_millis(200));
        println!("Posting task nr: {}", i);
        eventloop.post(task);
    }

    eventloop.start();
    println!("Starting tasks in 4 seconds");
    eventloop.post_timeout(task, Duration::from_secs(4));

    eventloop.stop_and_finish();
}
