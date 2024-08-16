#[allow(unused)]
fn main() {
    fn length_of_string(value: String) -> usize {
        value.len()
    }

    let s1 = String::from("Hello there");
    let len = length_of_string(s1);

    println!("The length is {}.", len);
}
