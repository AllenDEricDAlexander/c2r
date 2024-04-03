
fn main() {
    let mut ptr = Box::new(0);
    *ptr = 10;
    drop(ptr);
    // The second drop is intentionally ignored in Rust, because the ownership of the box has been moved after the first drop.
    // In C/C++, this would be a double free error.
    println!("{}", *ptr); // This will actually print the value before it is dropped, because println! takes ownership of the value.
}
