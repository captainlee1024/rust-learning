use closure::generate_workout;
use myiterator::*;

mod closure;
mod myiterator;

fn main() {
    //
    generate_workout(1, 1);

    //
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("A"),
        },
        Shoe {
            size: 11,
            style: String::from("B"),
        },
        Shoe {
            size: 10,
            style: String::from("C"),
        },
    ];

    println!("shoes in my size: {:?}", shoes_in_my_size(shoes, 10));

    //
    let mut counter = Counter::new();
    match counter.next() {
        Some(a) => println!("counter next: {}", a),
        None => println!("None"),
    }
}
