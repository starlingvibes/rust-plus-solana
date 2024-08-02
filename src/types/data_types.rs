fn main() {
    println!("TUPLES"); // TUPLES
    let my_tuple: (u8, bool, f32) = (8, true, 2.4);
    println!("{:?}", my_tuple);

    // accessing tuple elements
    println!("{}", my_tuple.0);
    println!("{}", my_tuple.2);

    // Tuple structs
    struct MyTuple(u8, bool, f32);

    let my_tuple_struct: MyTuple = MyTuple(8, true, 2.4);
    println!("{}", my_tuple_struct.1);

    // Name tuple using a type alias
    type MyTupleAlias = (u8, bool, f32);
    let my_tuple_alias: MyTupleAlias = (8, true, 2.4);
    println!("{}", my_tuple_alias.0);

    println!("\nSTRUCTS"); //STRUCTS
                           // Structs allow one give a name to each tuple field

    #[derive(Debug)]
    struct MyStruct {
        should_do_groceries: bool,
        birth_year: u32,
        height_in_meters: f64,
    }

    let my_struct = MyStruct {
        should_do_groceries: true,
        birth_year: 2024,
        height_in_meters: 23.4,
    };

    println!("{:?}", my_struct);

    println!("\nENUMS"); //ENUMS
                         // Enums are used to represent a fixed set of values known at compile time

    #[derive(Debug)]
    enum CardinalDirection {
        North,
        East,
        South,
        West,
    }

    let d = CardinalDirection::East;
    println!("{:?}", d);

    if let CardinalDirection::East = d {
        println!("We are going East");
    } else {
        println!("We are not going East but in some other direction")
    }

    println!("\nTAGGED UNIONS");
    // Tagged unions enable one to put different data types into a single collection. Basically, addind data to enum variants

    enum Shape {
        Square { side: f64 },
        Rectangle { height: f64, width: f64 },
        Circle { radius: f64 },
    }

    let s: Shape = Shape::Rectangle {
        height: 12.3,
        width: 24.6,
    };

    match s {
        Shape::Square { side } => {
            println!("A {}x{} square", side, side);
        }
        Shape::Rectangle { height, width } => {
            println!("A {}x{} rectangle", height, width);
        }
        Shape::Circle { radius } => {
            println!(
                "A circle with radius of {} and diameter of {}",
                radius,
                radius * 2.0
            );
        }
    }
}
