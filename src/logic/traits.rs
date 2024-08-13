#[allow(unused)]
fn main() {
    use std::fmt::Display;

    // These can be done because str values implement the Display traits
    let value = "Hello";
    println!("Regular: {}", value);
    println!("Padded: {:_>8}", value);

    // What the display trait and it's implementation for bool would look like
    // trait Display {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    // }

    // impl Display for bool {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    //         Display::fmt(if *self { "true" } else { "false" }, f)
    //     }
    // }

    // BOUNDS
    println!("\nBounds");

    struct MyStruct<A, B> {
        a: A,
        b: B,
    }

    // impl<A: Display, B: Display> MyStruct<A, B> {
    impl<A, B> MyStruct<A, B>
    where
        A: Display,
        B: Display,
    {
        fn print(&self) {
            println!("{} {}", self.a, self.b);
        }
    }

    let my_struct = MyStruct { a: 1, b: 2 };
    my_struct.print();

    // DERIVE
    println!("\nDerive");
    println!("{:?}", "Hello");
    println!("{:?}", vec!["Hello", "World"]);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let mick = Person {
        name: "Mick".to_string(),
        age: 18,
    };

    println!("{:?}", mick);
}
