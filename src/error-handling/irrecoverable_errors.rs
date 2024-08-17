#[allow(unused)]
fn main() {
    // panic!("An unrecoverable error occurred");

    std::fs::File::open("hello.txt").unwrap();

    std::fs::File::open("hello.txt").expect("Could not open hello.txt");

    match std::panic::catch_unwind(|| {
        panic!("An unrecoverable error occurred");
    }) {
        Ok(_) => println!("No errors occurred"),
        Err(_) => println!("The code panicked"),
    }
}
