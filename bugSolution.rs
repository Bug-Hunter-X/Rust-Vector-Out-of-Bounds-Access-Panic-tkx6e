fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before: {:?}", vec);

    // Safer way to access elements
    match vec.get(0) {
        Some(first) => match vec.get(1) {
            Some(second) => println!("First: {}, Second: {}", first, second),
            None => println!("Vector doesn't have a second element.")
        },
        None => println!("Vector is empty.")
    }
} 