use std::collections::HashMap;

fn main() {
    vector_collection_example();
    string_collection_example();
    hashmap_collection_example();
}

fn vector_collection_example() {
    println!("Starting vector example...\n");
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // panics if we access something outside of the vector range
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // won't panic if we try to access something outside of vector range, returns an empty optional instead which we can handle via the match expression
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        // iterate through v and grab immutable references to each value, can also specify as mutable references if vector is declared as mut
        println!("{i}");
    }
}

fn string_collection_example() {
    let s = String::from("hello");
    let s2 = String::from("world");
    let s3 = format!("{s} {s2}!"); // format concatenates the strings without moving ownership

    println!("{s}{s2}{s3}"); // this wouldn't have compiled if s or s2 ownership had moved to s3

    for c in s.chars() {
        println!("{c}");
    }

    for b in s.bytes() {
        println!("{b}");
    } // can iterate over strings by char or by byte
}

fn hashmap_collection_example() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    println!("Blue score is {blue_score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // count is a mutable reference to an i32, so in order to update it we need to dereference it with the * operator
    }

    println!("{map:?}");
}
