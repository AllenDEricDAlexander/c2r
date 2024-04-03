
use std::thread;
use std::time::Duration;

static mut DATA: i32 = 0;

fn increment(v: *const std::ffi::c_void) {
    for _ in 0..1000000 {
        unsafe {
            DATA += 1;
        }
    }
}

fn main() {
    let t1 = thread::spawn(move || {
        increment(std::ptr::null());
    });
    let t2 = thread::spawn(move || {
        increment(std::ptr::null());
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
