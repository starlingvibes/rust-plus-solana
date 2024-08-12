#[allow(unused)]
fn main() {
    use std::fmt::Display;
    use std::ops::Add;

    struct Sequence3<T> {
        first: T,
        second: T,
        third: T,
    }

    // impl<T> Sequence3<T> where T: PartialEq {
    impl<T: PartialEq> Sequence3<T> {
        pub fn new(first: T, second: T, third: T) -> Self {
            Self {
                first,
                second,
                third,
            }
        }

        pub fn all_same(&self) -> bool {
            self.first == self.second && self.second == self.third
        }
    }

    impl<T> Sequence3<T>
    where
        T: Copy + Add<Output = T>,
    {
        pub fn sum(&self) -> T {
            self.first + self.second + self.third
        }
    }

    let seq = Sequence3::new(1, 2, 3);
    println!("{}", seq.all_same());
    println!("{}", seq.sum());

    // USING MULTIPLE GENERIC TYPE PARAMETERS
    println!("\nUsing multiple generic type parameters");
    #[derive(Debug)]
    struct MyStruct<A, B> {
        a: A,
        b: B,
    }

    #[derive(Debug)]
    enum MyEnum<A, B> {
        A(A),
        B(B),
    }

    let my_struct = MyStruct { a: 1, b: 2 };
    let my_enum = MyEnum::<i32, _>::B("Hello");
    let my_enum2 = MyEnum::<_, i32>::A(1); // ?

    println!("{:?}", my_struct);
    println!("{:?}", my_enum);
    println!("{:?}", my_enum2);

    // GENERIC FUNCTIONS
    // It is possibe to write free functions that accept generic types too
    println!("\nGeneric functions");
    fn say_hello<T: Display>(value: &T) {
        println!("Hello {}", value);
    }

    say_hello(&true);
    say_hello(&String::from("world"));
    say_hello(&1337);
}
