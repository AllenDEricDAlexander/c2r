
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
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
        .expect("Failed to create/open temp file");
    let file_size = 4096;
    let mut file_contents = vec![0; file_size];

    temp_file.set_len(file_size as u64).expect("Failed to set file length");

    temp_file.seek(SeekFrom::Start(1024 as u64)).expect("Failed to seek in file");
    temp_file.write_all(b"haha, I'm fine").expect("Failed to write to file");

    // Shared memory mapping is not directly available in Rust,
    // so we'll simulate it using a thread to read from the file.
    let shared_memory = SharedMemory::new(file_size);
    let handle = shared_memory.get_handle();

    thread::spawn(move || {
        let mut data = vec![0; file_size];
        shared_memory.read_exact(&mut data).expect("Failed to read from shared memory");
        println!("Message read from shared memory: {}", String::from_utf8_lossy(&data));
    });

    thread::sleep(Duration::from_secs(2));

    // Unmap shared memory in the other thread
    if let Ok(mut guard) = shared_memory.try_lock() {
        guard.unmap();
    }

    drop(temp_file);
}

struct SharedMemory {
    handle: std::fs::File,
    offset: u64,
}

impl SharedMemory {
    fn new(size: usize) -> Self {
        let file = std::fs::File::open("temp").unwrap();
        let offset = file.metadata().unwrap().len();

        SharedMemory { handle: file, offset }
    }

    fn read_exact(&self, data: &mut [u8]) -> io::Result<()> {
        let file = &self.handle;
        let offset = self.offset;

        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(data)
    }

    fn get_handle(&self) -> std::fs::File {
        self.handle.try_clone().unwrap_or_else(|e| {
            panic!("Failed to clone shared memory file: {}", e)
        })
    }

    fn try_lock(&self) -> std::io::Result<std::sync::MutexGuard<'_, Self>> {
        std::sync::Mutex::new(self).try_lock()
    }

    fn unmap(self: Box<Self>) {
        // In Rust, unmapping is typically handled by dropping the reference.
        // Since we're in a thread, we need to explicitly drop the boxed instance.
        drop(self);
    }
}
