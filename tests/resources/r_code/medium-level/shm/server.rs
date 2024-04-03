
use std::io::{self, OpenOptions};
use std::fs::{self, OpenMode};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;

fn main() {
    let temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("temp")
        .expect("Failed to open temp file");

    let len = temp_file.seek(io::SeekFrom::End(0)).expect("Failed to get file length");

    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            len as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            temp_file.as_raw_fd(),
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        let err = io::Error::last_os_error();
        eprintln!("mmap: {}", err);
        std::process::exit(1);
    }

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("{}", unsafe {
                std::ffi::CStr::from_ptr((*ptr).add(1024) as *const _)
                    .to_string_lossy()
                    .into_owned()
            });
        }
    });

    // Release the memory mapping
    let ret = unsafe { libc::munmap(ptr, len as libc::size_t) };

    if ret == -1 {
        let err = io::Error::last_os_error();
        eprintln!("munmap: {}", err);
        std::process::exit(1);
    }
}
