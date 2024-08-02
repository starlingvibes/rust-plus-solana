#[allow(unused)]
fn main() {
    let should_print: bool = true;
    let value = 10;
    if (should_print && value > 9) {
        println!("Value greater than 9");
    }

    println!("\nLOOPING");
    let mut i = 10;
    loop {
        if i == 0 {
            break;
        }
        println!("{}...", i);
        i -= 1;
    }
    println!("Launch");

    let mut i = 10;
    while i != 0 {
        println!("{}...", i);
        i -= 1;
    }
    println!("Launch");

    for i in (1..=10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{}...", i);
    }
    println!("Launch")
}
