
use std::io::{self, Read, Write};
use std::net::{TcpStream, UnixStream};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::str;
use std::sys::socket;
use std::thread;

const SOCKET_NAME: &str = "/tmp/DemoSocket";
const BUFFER_SIZE: usize = 128;
const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <number>", args[0]);
        return EXIT_FAILURE;
    }

    let number = args[1].parse::<i32>().expect("Failed to parse number");

    let mut stream = match UnixStream::connect(SOCKET_NAME) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return EXIT_FAILURE;
        }
    };

    let mut buffer = [0; BUFFER_SIZE];

    // Send number
    stream.write_all(&number.to_le_bytes()).expect("Failed to write to socket");
    println!("No of bytes sent = {}, data sent = {}", stream.write_all(&number.to_le_bytes()).unwrap(), number);

    // Receive result
    stream.read_exact(&mut buffer).expect("Failed to read from socket");
    println!("Result = {}", str::from_utf8(&buffer).unwrap());

    0
}
