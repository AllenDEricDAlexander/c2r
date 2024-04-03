
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::ffi::CString;
use std::thread;
use nix::sys::wait::waitpid;
use nix::sys::msg::{msgsnd, msgrcv, MsgFlags, MsgType};
use nix::sys::ipc::{msgget, IPC_PRIVATE, IPC_CREAT, 0o666};
use nix::unistd::{fork, ForkResult};

const MY_TYPE: i32 = 9527;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let msgid = msgget(IPC_PRIVATE, IPC_CREAT | 0o666)?;

    match fork() {
        Ok(0) => {
            // Child process
            let buffer = SharedMemoryBuffer::new(msgid);
            println!("Child process is waiting for msg:");
            buffer.receive_message()?;
            println!("Child process read from msg: {}", buffer.read_text());
        }
        Ok(pid) => {
            // Parent process
            thread::sleep(Duration::from_secs(1));
            let buffer = SharedMemoryBuffer::new(msgid);
            buffer.write_text("Hello, World!");
            buffer.write_number(42);
            buffer.send_message(MY_TYPE)?;
            waitpid(pid, None)?;
        }
        Err(e) => {
            eprintln!("fork error: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}

struct SharedMemoryBuffer {
    msgid: i32,
    buffer: Vec<u8>,
}

impl SharedMemoryBuffer {
    fn new(msgid: i32) -> Self {
        SharedMemoryBuffer {
            msgid,
            buffer: vec![0; 100],
        }
    }

    fn receive_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg = std::mem::MaybeUninit::<msg::Message>::uninit();
        let flags = MsgFlags::empty();
        msgrcv(self.msgid, msg.as_mut_ptr(), self.buffer.len() as _, MY_TYPE, flags)?;
        let msg = msg.assume_init();

        self.buffer[0..msg.mtext.len()].copy_from_slice(msg.mtext.as_bytes());
        self.buffer[msg.mtext.len()..].copy_from_slice(msg.mtext.as_bytes());

        Ok(())
    }

    fn write_text(&mut self, text: &str) {
        let text_len = text.len();
        self.buffer[100 - text_len..].copy_from_slice(text.as_bytes());
    }

    fn write_number(&mut self, number: i32) {
        let number_bytes = number.to_ne_bytes();
        let number_len = number_bytes.len();
        self.buffer[100 - number_len - 10..100 - number_len].copy_from_slice(&number_bytes);
    }

    fn send_message(&self, mtype: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg = msg::Message {
            mtype,
            mtext: CString::new(self.buffer).unwrap().into_raw(),
            mtext_size: self.buffer.len() as _,
            ..Default::default()
        };
        msgsnd(self.msgid, &msg, msg.size() as _, 0)
    }

    fn read_text(&self) -> String {
        String::from_utf8(self.buffer[100 - self.buffer.len()..].to_vec()).unwrap()
    }
}
