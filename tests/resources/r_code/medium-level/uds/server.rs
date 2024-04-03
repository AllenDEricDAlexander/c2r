
use std::io::{self, Read};
use std::net::{self, UnixStream};
use std::os::unix::io::AsRawFd;
use std::str;

const SOCKET_NAME: &str = "/tmp/DemoSocket";
const BUFFER_SIZE: usize = 128;

fn main() -> io::Result<()> {
    let listener = UnixStream::bind(SOCKET_NAME)?;
    println!("Master socket created");

    loop {
        let (mut stream, _) = listener.accept()?;
        println!("Connection accepted from client");

        let mut buffer = [0; BUFFER_SIZE];
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            let data: i32 = i32::from_ne_bytes(buffer[..bytes_read].try_into().unwrap());
            let mut result = 0;
            result += data;
        }

        let result_str = format!("Result = {}", result);
        stream.write_all(result_str.as_bytes())?;
        println!("sending final result back to client");

        stream.shutdown(net::Shutdown::Both)?;
        println!("connection closed..");
    }
}
