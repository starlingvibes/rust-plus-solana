#[allow(unused)]
fn main() {
    // DECLARATIVE MACROS
    println!("Declarative Macros");

    macro_rules! create_vec {
    ( $( $item:expr ),* ) => {
        {
            let mut result = Vec::new();
            $(
                result.push($item);
            )*
            result
            }
        }
    }
    let items = create_vec!(1, 2, 3);
    println!("{items:?}");

    // PROCEDURAL MACROS
    // There are 3 types: custom derive macros, attribute-like macros and function-like macros
    println!("\nProcedural Macros");

    #[derive(Default, Clone, PartialEq, Debug, Eq)]
    struct MyType {
        name: String,
        items: Vec<i32>,
    }

    let v1 = MyType::default();
    let v2 = v1.clone();
    assert!(v1.name.is_empty());
    assert!(v2.items.is_empty());
    assert_eq!(&v1, &v2);
    println!("{v1:#?}");

    // ATTRIBUTE-LIKE MACROS
    // These can be applied to any item such as functions, constants and type definitions
    // #[my_attr_macro]
    fn x() {}

    // #[my_attr_macro]
    const Y: u32 = 1;

    // #[my_attr_marco]
    struct Z;
}
