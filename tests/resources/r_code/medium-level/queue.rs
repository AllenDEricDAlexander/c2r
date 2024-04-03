
use std::cmp::Ordering;
use std::io;
use std::mem;
use std::ptr;

type ElementType = i32;

fn error(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    std::process::exit(1);
}

fn fatal_error(msg: &str) -> ! {
    eprintln!("Fatal Error: {}", msg);
    std::process::exit(1);
}

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl QueueRecord {
    fn new(max_elements: usize) -> Self {
        if max_elements < 5 {
            error("Queue size is too small");
        }

        let array = vec![0; max_elements];

        QueueRecord {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn succ(&self, value: usize, queue: &Self) -> usize {
        if let Some(new_value) = value.checked_add(1) {
            if new_value == queue.capacity {
                0
            } else {
                new_value
            }
        } else {
            0
        }
    }
}

fn create_queue(max_elements: usize) -> Box<QueueRecord> {
    let q = Box::new(QueueRecord::new(max_elements));
    q
}

fn make_empty(q: &mut QueueRecord) {
    q.size = 0;
    q.front = 1;
    q.rear = 0;
}

fn dispose_queue(q: Box<QueueRecord>) {
    if !q.array.is_empty() {
        let _ = std::mem::drop(q.array);
    }
    let _ = std::mem::drop(q);
}

fn enqueue(x: ElementType, q: &mut QueueRecord) {
    if q.is_full() {
        error("Full queue");
    }

    q.size += 1;
    q.rear = q.succ(q.rear, q);
    q.array[q.rear] = x;
}

fn front(q: &QueueRecord) -> ElementType {
    if q.is_empty() {
        error("Empty queue");
    }

    q.array[q.front]
}

fn dequeue(q: &mut QueueRecord) {
    if q.is_empty() {
        error("Empty queue");
    }

    q.size -= 1;
    q.front = q.succ(q.front, q);
}

fn front_and_dequeue(q: &mut QueueRecord) -> ElementType {
    let x = front(q);
    dequeue(q);
    x
}

fn main() {
    let mut q = create_queue(12);

    for i in 0..10 {
        enqueue(i, &mut q);
    }

    while !q.is_empty() {
        println!("{:?}", front(&q));
        dequeue(&mut q);
    }

    for i in 0..10 {
        enqueue(i, &mut q);
    }

    while !q.is_empty() {
        println!("{:?}", front(&q));
        dequeue(&mut q);
    }

    dispose_queue(q);
}