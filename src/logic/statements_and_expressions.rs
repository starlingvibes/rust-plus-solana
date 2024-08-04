#[allow(unused)]
fn main() {
    let quantifier;
    let brownies_eaten = 5;
    if brownies_eaten == 0 {
        quantifier = "no brownies";
    } else {
        quantifier = "at least one brownie";
    }
    println!("I had {quantifier} today");

    let quantifier = if brownies_eaten == 0 {
        "no brownies"
    } else {
        "at least one brownie"
    };
    println!("I had {quantifier} today");

    let second_quantifier = match brownies_eaten {
        0 => "no brownies",
        1 => "a brownie",
        _ => "multiple brownies",
    };
    println!("I had {second_quantifier} today");
}
