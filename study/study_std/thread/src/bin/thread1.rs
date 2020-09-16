use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_micros(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_micros(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("here is a vector:{:?}", v);
    });

    handle1.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let handle3 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    // let val = rx.try_recv();

    loop {
        if let Ok(ret) = rx.try_recv() {
            println!("{:?}", ret);
            break;
        } else {
            println!("got nothing");
        }
        thread::sleep(Duration::from_millis(300));
    }

    handle3.join().unwrap();

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("thread-{} added", i);
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
