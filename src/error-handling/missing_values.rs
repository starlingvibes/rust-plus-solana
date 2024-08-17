#[allow(unused)]
fn main() {
    // Dealing with missing values
    println!("\nDealing with missing values");

    fn first_element1<T>(array: &[T]) -> &T {
        if array.len() > 0 {
            &array[0]
        } else {
            unimplemented!("Missing value");
        }
    }

    let array: [i32; 2] = [1, 2];
    let value = first_element1(&array);
    println!("The first element in {array:?} is {:?}", value);

    fn first_element<T>(array: &[T]) -> Option<&T> {
        if array.len() > 0 {
            Option::Some(&array[0])
        } else {
            Option::None
        }
    }

    let array: [i32; 2] = [1, 2];
    let empty_array: [i32; 0] = [];
    let value = first_element(&array);
    let second_value = first_element(&empty_array);
    println!("The first element in {array:?} is {:?}", value);
    println!("The first element in {empty_array:?} is {:?}", second_value);

    // Simple Book API
    #[derive(Debug)]
    struct APIBook {
        title: String,
        description: Option<String>,
    }

    #[derive(Debug)]
    struct Book {
        title: String,
        description: String,
    }

    let api_books: Vec<APIBook> = vec![
        APIBook {
            title: "Harry Potter and the Seven Kings".to_string(),
            description: Some("The best book yet!".to_string()),
        },
        APIBook {
            title: "Merlin and the playful kids".to_string(),
            description: None,
        },
    ];

    println!("{:?}", api_books);

    // let books: Vec<Book> = api_books
    //     .into_iter()
    //     .filter_map(|api_book| {
    //         let APIBook { title, description } = api_book;

    //         let description = match description {
    //             Some(description) => description,
    //             None => return None,
    //         };

    //         Some(Book { title, description })
    //     })
    //     .collect::<Vec<_>>();

    // println!("{books:?}");

    // Doing the above succintly using the error propagation operator

    let my_books: Vec<Book> = api_books
        .into_iter()
        .filter_map(|api_book| {
            Some(Book {
                title: api_book.title,
                description: api_book.description?,
            })
        })
        .collect::<Vec<_>>();

    println!("{my_books:?}");
}
