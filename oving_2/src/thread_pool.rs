use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Pool {
    workers: u32,
    tasks: Arc<Mutex<VecDeque<fn()>>>,
    condvar: Arc<(Mutex<bool>, Condvar)>,
    run_threads: Vec<JoinHandle<()>>,
    running: Arc<Mutex<bool>>,
}

impl Pool {
    pub fn new(workers: u32) -> Self {
        Self {
            workers,
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            condvar: Arc::new((Mutex::new(false), Condvar::new())),
            run_threads: Vec::new(),
            running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn start(&mut self) {
        //check if start has been run
        if !self.run_threads.is_empty() {
            return;
        }

        //Creating threads of amount workers
        for _ in 0..self.workers {
            let condvar_c = Arc::clone(&self.condvar);
            let running_c = Arc::clone(&self.running);
            let tasks_c = Arc::clone(&self.tasks);

            self.run_threads.push(thread::spawn(move || {
                //while threads should run
                while *running_c.lock().unwrap() || !tasks_c.lock().unwrap().is_empty() {
                    //start wait for signal
                    let (lock, cvar) = &*condvar_c;
                    {
                        let mut started = lock.lock().unwrap();
                        while !*started {
                            started = cvar.wait(started).unwrap();
                        }
                    }

                    //while tasks left
                    while !tasks_c.lock().unwrap().is_empty() {
                        println!("Checking for task");
                        let mut task: Option<fn()> = None;

                        //get task
                        {
                            let mut tasks = tasks_c.lock().unwrap();
                            if !tasks.is_empty() {
                                task = Some(tasks.pop_front().unwrap());
                            }
                        }

                        //run task
                        if let Some(current_task) = task {
                            current_task();
                        }
                    }
                    *lock.lock().unwrap() = false;
                }
                println!("Thread {:?} dying", thread::current().id())
            }));
        }
    }

    pub fn stop_and_finish(mut self) {
        *self.running.lock().unwrap() = false;

        while !self.run_threads.is_empty() {
            self.run_threads.pop().unwrap().join().unwrap();
        }

        self._notify_one();
    }

    pub fn post(&self, task: fn()) {
        self.tasks.lock().unwrap().push_back(task);

        self._notify_all();
    }

    pub fn post_timeout(&self, task: fn(), dur: Duration) {
        thread::sleep(dur);
        self.post(task);
    }

    pub fn _notify_one(&self) {
        let (lock, cvar) = &*self.condvar;
        *lock.lock().unwrap() = true;
        cvar.notify_one();
    }

    fn _notify_all(&self) {
        let (lock, cvar) = &*self.condvar;
        *lock.lock().unwrap() = true;
        cvar.notify_all();
    }
}
