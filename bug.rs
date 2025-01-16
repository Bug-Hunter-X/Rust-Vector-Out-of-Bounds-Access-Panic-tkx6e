fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before: {:?}", vec);

    // This will cause a runtime error if the vector is empty
    let first = vec.get(0).unwrap();
    let second = vec.get(1).unwrap();

    println!("First: {}, Second: {}", first, second);
}