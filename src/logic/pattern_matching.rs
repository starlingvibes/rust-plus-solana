#[allow(unused)]
fn main() {
    struct Plant {
        flowering: bool,
        mass: f64,
    }
    let plant = Plant {
        flowering: true,
        mass: 34.4,
    };
    let Plant { flowering, mass } = plant;
    println!("{flowering}, {mass}");

    enum Meal {
        FishAndChips { chip_proportion: f64 },
        Hamburger { vegetarian: bool },
    }
    let meal = Meal::Hamburger { vegetarian: false };
    // let Meal::Hamburger { vegetarian } = meal;
    // ^ The compiler rejects the code because the pattern is refutable, meaning that there exists values that cannot be matched

    if let Meal::Hamburger { vegetarian: true } = meal {
        println!("I had a vegetarian hamburger!");
    }

    println!("\nMATCH");
    for n in 0..=5 {
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            3 | 4 => println!("It was a bit more than two"),
            high if high >= 5 => println!("It was a high number"),
            other => println!("It was {other}"),
        }
    }
    // Matching enums
    match meal {
        Meal::FishAndChips { chip_proportion } => {
            if chip_proportion > 0.5 {
                println!("I had fish and plenty of chips");
            } else if chip_proportion < 0.5 {
                println!("I had chips and plenty of fish");
            } else {
                println!("I had fish and chips");
            }
        }
        Meal::Hamburger { vegetarian } => {
            if vegetarian {
                println!("I had a vegetarian hamburger");
            } else {
                println!("I had a meaty hamburger");
            }
        }
    }
    // Using match guards to match against values inside the enum variants
    match meal {
        Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
            println!("I had fish and plenty of chips")
        }
        Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
            println!("I had chips and plenty of fish")
        }
        Meal::FishAndChips { chip_proportion } => println!("I had fish and chips"),
        Meal::Hamburger { vegetarian: true } => {
            println!("I had a vegetarian hamburger")
        }
        Meal::Hamburger { vegetarian: false } => {
            println!("I had a meaty hamburger")
        }
    }

    let mut meal = Meal::FishAndChips {
        chip_proportion: 0.6,
    };
    while let Meal::FishAndChips { chip_proportion } = meal {
        println!(
            "Having fish and chips with chip proportion of {:.2}",
            chip_proportion
        );
        if chip_proportion > 0.3 {
            meal = Meal::FishAndChips {
                chip_proportion: chip_proportion - 0.2,
            }
        } else {
            meal = Meal::Hamburger { vegetarian: true }
        }
    }
    println!("I am so done with fish and chips");

    // Working with irrefutable patterns using the for loop
    let tuples: [(usize, &'static str); 3] = [(1, "red"), (2, "green"), (3, "blue")];
    for (numbering, colour) in tuples {
        println!("Colour #{} is {}", numbering, colour);
    }
    let colours: [&'static str; 3] = ["red", "green", "blue"];
    for (index, colour) in colours.iter().enumerate() {
        let numbering = index + 1;
        println!("Colour #{numbering} is {colour}");
    }
}
