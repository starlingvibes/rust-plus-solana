#[allow(unused)]
fn main() {
    fn length_of_string(value: String) -> usize {
        value.len()
    }

    let s1 = String::from("Hello there");
    let len = length_of_string(s1);

    println!("The length is {}.", len);

    // RETURNING OWNERSHIP
    println!("\nReturning ownership");

    fn length_of_string2(value: String) -> (String, usize) {
        let len = value.len();
        (value, len)
    }

    let s2 = String::from("Hello there");

    let (s2, len) = length_of_string2(s2);
    println!("The length of {s2:?} is {}.", len);
    // NB: the ownership of s2 is returned to the caller and there is no need to duplicate but it's not easy to read especially if there are multiple parameters

    // PASS BY REFERENCE
    println!("\nPass by reference");

    fn length_of_string3(value: &String) -> usize {
        value.len()
    }

    let s3 = String::from("Hello there");

    let len = length_of_string3(&s3);
    println!("The length of {:?} is {}.", s3, len);

    // PASS BY MUT REFERENCE
    println!("\nPass by mutable reference");
    fn append_world(value: &mut String) {
        value.push_str(" world");
    }

    let mut s4 = String::from("Hello");
    append_world(&mut s4);
    println!("The value is now {s4}");

    // DATA RACES
    println!("\nData races");

    enum StringOrInt {
        Str(String),
        Int(i32),
    }

    let mut x: StringOrInt = StringOrInt::Str("Hi".to_string());
    let rx: &mut StringOrInt = &mut x;

    if let StringOrInt::Str(ref insides) = x {
        // The next line should not be allowed, because it overwrites the value
        // in `x` while you hold a reference to what is currently inside of `x`.
        *rx = StringOrInt::Int(1);
        // You are trying to print the value inside `x` as a `String`, but the
        // `String` has been overwritten by `1: i32`. You can not interpret the
        // bytes of the `i32` as a `String`!
        println!("x says: {}", insides);
    }

    // Dangling references
    println!("\nDangling references");

    let reference_to_nothing = dangle();

    fn dangle() -> &String {
        // You create a new string and assign it to `s`.
        let s = String::from("hello"); // s is a new String

        // You return a reference to the String stored in `s`.
        &s
        // The function ends and the string `s` goes out of scope.
        // Because `s` goes out of scope, its value gets dropped.
        // When a String is dropped, the underlying memory gets deallocated.
    }

    // Non-Lexical lifetimes
    println!("\nNon-lexical lifetimes");
    // A reference's scope starts from where it is introduced and continues through the last time that reference is used.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
