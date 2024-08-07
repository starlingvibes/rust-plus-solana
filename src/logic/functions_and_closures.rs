#[allow(unused)]
fn main() {
    fn f(n: u32) -> u32 {
        fn g(n: u32) -> u32 {
            n + 1
        }
        g(n * 2)
    }
    println!("{}", f(3));

    println!("\nAssociated functions");
    // You can attach functions to data types such as struct and enum
    struct X(&'static str);
    impl X {
        fn associated_fn() -> &'static str {
            "Just an associated function"
        }

        fn method(self: &Self) -> &'static str {
            self.0
        }
    }

    println!("{}", X::associated_fn());
    let x = X("Instantiating an instance of X");
    println!("{}", X::method(&x));
    println!("{}", x.method());

    println!("\nClosures");
    // Closures are similar to functions, except that they have the ability to "capture their environment"
    let c = |x| x * 2;
    println!("{}", c(5));

    let mut n = 0;
    let mut c = |x| {
        n += 1;
        x + n - 1
    };
    println!("{}", c(2));
    println!("{}", c(2));
    println!("{}", c(2));

    let a: [u32; 3] = [2, 5, 6];
    let y: u32 = a.iter().map(|d| d * 2).sum();
    println!("The sum of the elements in {:?} when doubled is {}", a, y);
}
