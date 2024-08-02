fn main() {
    println!("\nSCOPING");
    // Rust allows you use variables declared in outer scopes but variables do not leak out of their scope
    let x = 2;
    println!("{}", x);
    {
        let y = 3;
        println!("{}, {}", x, y);
    }
    println!("{}", x);

    println!("\nSHADOWING");
    // It is allowed to redefine a variable with the same name
    let x = 4;
    let x = 8;
    {
        let x = 5;
    }
    println!("{}", x);

    println!("\nPATTERNS");
    // The let statement is quite powerful and allows one provide a pattern instead of just the variable name
    let (x, y) = (2, 3); // destructuring a tuple
    #[derive(Debug)]
    struct Person {
        name: &'static str,
        age: u16,
        likes_brownies: bool,
    }

    let p = Person {
        name: "Chidera Anichebe",
        age: 32,
        likes_brownies: true,
    };
    // destructuring a struct
    let Person { name, age, .. } = p;
    println!("{} is {} year(s) old", name, age);
}
