use std::{thread, sync::{Arc, Mutex}};

fn main() {
    let mut threads = Vec::new();
    let primes: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    
    //Change bound here 
    let bound = Arc::new(vec![1, 100]);
    //Change thread count here
    let thread_count = 10;

    for i in 0..thread_count {
        let bound_clone = bound.clone();
        let primes_clone = primes.clone();
        let thread_count_clone = thread_count.clone();
        threads.push(thread::spawn(move || {
            // i is copied into thread
            let thread_bound = find_bound(i as i32, bound_clone, thread_count_clone);
            println!("I am thread {} and my bound is {}, {}", i, thread_bound[0], thread_bound[1]);
            find_primes(thread_bound, primes_clone);
        }));
    }

    for thread in threads {
        let _ = thread.join(); // let _ means that the return value should be ignored
    }
    
    primes.lock().unwrap().sort();

    println!("The Primes found between {} and {} are:\n{:?}", bound[0], bound[1], primes.lock().unwrap());
}

fn find_bound(i: i32, bound: Arc<Vec<i32>>, thread_count: i32) -> Vec<i32>{
    let mut thread_bound: Vec<i32> = Vec::new();
    let bound_size = (bound[1] - bound[0] + 1)/thread_count;
    
    thread_bound.push(bound[0] + i * bound_size);
    if bound[0] + i * bound_size + bound_size - 1 > bound[1] || (i + 1) == thread_count{
        thread_bound.push(bound[1]);
    }else{
        thread_bound.push(bound[0] + i * bound_size + bound_size - 1);
    }

    thread_bound
}

fn find_primes(thread_bound: Vec<i32>, primes: Arc<Mutex<Vec<i32>>>){
    for i in  thread_bound[0]..thread_bound[1]{
        if is_prime(i as i32){
            primes.lock().unwrap().push(i as i32);
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    if n < 25 {
        return true;
    }
    let mut i: i32 = 5;
    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}