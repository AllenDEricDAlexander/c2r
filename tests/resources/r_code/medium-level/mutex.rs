
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, mpsc};
use std::cell::RefCell;

fn thread1_function(_arg: &()) {
    let mut i = 0;
    loop {
        let mutex = Mutex::new(());
        {
            let _guard = mutex.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
                thread::sleep(Duration::from_secs(1));
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(_arg: &()) {
    let mut i = 0;
    thread::sleep(Duration::from_secs(1));
    loop {
        let mutex = Mutex::new(());
        {
            let _guard = mutex.lock().unwrap();
            for _ in 0..10 {
                println!("Good moring");
                thread::sleep(Duration::from_secs(1));
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let handles = vec![
        thread::spawn(thread1_function),
        thread::spawn(thread2_function),
    ];

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("thread panicked: {:?}", e);
        }
    }
    println!("The thread is over, process is over too.");
}
