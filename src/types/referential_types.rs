fn main() {
    println!("IMMUTABLE REFERENCES");
    struct Config {
        port: u32,
    }
    let config: Config = Config { port: 1337 };
    let config_reference: &Config = &config;
    println!("Application running on port {}", config_reference.port);
    // It's fine to have multiple immutable references to a value at the same time
    let val: u16 = 10;
    let r1: &u16 = &val;
    let r2: &u16 = &val;
    println!("{} should be the same as {}", r1, r2);

    println!("\nMUTABLE REFERENCES");
    struct Config2 {
        port: u16,
    }
    let mut config2: Config2 = Config2 { port: 4000 };
    let mutable_config_reference: &mut Config2 = &mut config2;
    mutable_config_reference.port = 1337;
    println!(
        "Application running on port {}",
        mutable_config_reference.port
    );
    // It's not fine to have more than one mutable reference to a value simultaneously
    let mut val = 10;
    let r1 = &mut val;
    // let r2 = &mut val; /// ERROR
    *r1 = 5;
    // *r2 = 6;
    println!("{}", r1);

    println!("\nDEFERENCING");
    // The deferencing operator (*) is used to access the value behind a reference
    let val: i32 = 10;
    let r1: &i32 = &val;
    let val2: i32 = *r1;
    println!("{}", val2);

    println!("\nCOPY");
    // A copyable type is one where we can create a new value simply by copying all the bits. Not all types are copyable (e.g String)
    let val: String = "Hello".to_string();
    let r1: &String = &val;
    // Compilation error
    // let val2: String = *r1;
}
