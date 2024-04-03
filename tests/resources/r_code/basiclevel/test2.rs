
fn print_values<T: std::fmt::Display>(values: &[T]) {
    for value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = [10, 20, 30];
    print_values(&values);
}
