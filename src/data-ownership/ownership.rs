#[allow(unused)]
fn main() {
    // Print the stack-size of a String.
    println!(
        "The size of a `String` is {}",
        std::mem::size_of::<String>()
    );
    // Create a String with a capacity of 4.
    let mut hello = String::with_capacity(4);
    // Print how the String is represented on the stack.
    print_string_stack_data(&hello);
    // Append the text "Hello!" to the (currently empty) String.
    hello.push_str("Hello!");
    // The capacity and length should have changed, and maybe the pointer.
    print_string_stack_data(&hello);

    // Below wouldn't work as it would be a move operation since String does not implement the Copy trait.
    // let a = String::from("Hello");
    let a = 10;
    let b = a;
    println!("a = {a}");

    let a = String::from("Hello");
    let b = a.clone();
    println!("a = {a}, b = {b}");
}
// Learning about unsafe Rust is out of scope so ignore this function.
fn print_string_stack_data(value: &String) {
    let ptr = value as *const _ as *const usize;
    println!("pointer  {0:16} 0x{0:016X}", unsafe { *ptr });
    println!("capacity {0:16} 0x{0:016X}", unsafe { *ptr.offset(1) });
    println!("length   {0:16} 0x{0:016X}", unsafe { *ptr.offset(2) });
}
