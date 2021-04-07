// Project: rust-collections
// Author: Greg Folker

// An `enum` can be used to store multiple types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
	println!("Hello, World!");

    // The first collection type in Rust is a vector
    //
    // Vectors allow you to store more than one value into a single
    // data structure such that all of the values are next to each
    // other in memory
    //
    // To initialize an empty vector, use `Vec::new` with type annotation
    let _v1: Vec<i32> = Vec::new();

    // Initializing a static vector with some values
    let _v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    // Use `push` to add elements to an existing vector
    v3.push(3);
    v3.push(4);
    v3.push(5);
    v3.push(6);
    v3.push(7);

    // A vector is freed when it goes out of scope
    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with _v
    } // <- _v goes out of scope and is freed here

    // There are two methods of accessing elements in a vector in Rust
    // Using indexing syntax or the `get` method
    let v = vec![1, 2, 3, 4, 5];

    // Using an index
    let third_element: &i32 = &v[2];
    println!("The third element of v is {}!", third_element);

    // Using the `get` method
    match v.get(2) {
        Some(third_element) => println!("The third element of v is {}!", third_element),
        None => println!("There is no third element in v"),
    }

    // Line 58 is a compiler error in Rust because we are trying to access an element
    // outside of the bounds of our vector
    // let does_not_exist = &v[100];

    // This is allowed, however, because the `get` method just returns `None`
    let _does_not_exist = v.get(100);

    // Rust has a 'borrow-checker' to ensure that elements do not change value
    // if they are being held somewhere else
    // Therefor, the following code is a compiler error in Rust
    //
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0]; // A reference to `v` is being held by `first`
    //
    // Attempting to add an element to a vector while a reference to it is held elsewhere (Line 60)
    // v.push(6);

    // Iterating over vectors can be done using a `for` loop
    for i in &v {
        println!("{}", i);
    }

    // Modifying vectors can also be done using a `for` loop, so long
    // as the vector is mutable
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{}", i);
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // The second collection type in Rust is the `String` type
    //
    // The String type, which is provided by Rust’s standard library rather than coded into
    // the core language, is a growable, mutable, owned, UTF-8 encoded string type
    //
    // Many of the same operations available to vectors are available to Strings as well
    let mut _s1 = String::new();
    let data = "initial data";

    _s1 = data.to_string();

    println!("s1 is now '{}'", _s1);

    let mut s1 = String::from("foo");
    let s2 = "bar";

    // Appending to a string can be done using the `push_str` method
    s1.push_str(s2);

    println!("s2 is now '{}'", s2);

    // Concatenating String variables can be done using the `+` operator
    // or the `format!` macro
    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");

    // Note, s3 has been moved here and can no longer be used after Line 120
    let s5 = s3 + &s4;

    println!("s5 is now '{}'", s5);

    // The `+` operator gets unweildy with lots of values
    // This is where the `!format` macro comes in handy
    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");

    let s = format!("{}-{}-{}", s6, s7, s8);

    println!("s is now '{}'", s);

    // A `String` is a wrapper over a Vec<u8> object
    //
    // The length, or size, of a String is how many bytes it takes
    // to encode the value in UTF-8. Because of this, Rust does not allow
    // you to index directly into strings (e.g., `s[10]`) even if the index
    // seemingly falls within the length of the String object. You have to
    // be more specific by using a String slice
    //
    // String slices should still be used with caution as they can crash
    // your program if you try to read from an index range that is not
    // on a byte, or char, boundary

    let hello = "Здравствуйте";

    // Here, `s9` will be the first four bytes of `hello`
    // Since each of these characters requires 2 bytes to be
    // UTF-8 encoded, the result will be 'Зд'
    let s9 = &hello[0..4];

    println!("The first four bytes of 'hello' are encoded as '{}'", s9);

    // There are methods to iterate over Strings to avoid invalid access
    // errors at runtime using String slices
    let mut char_idx = 0;
    for c in "नमस्ते".chars() {
        char_idx += 1;
        println!("Char {} is {}", char_idx, c);
    }

    // The raw bytes can be printed as well with the `bytes()` method
    let mut byte_idx = 0;
    for b in "नमस्ते".bytes() {
        byte_idx += 1;
        println!("Byte {} is {}", byte_idx, b);
    }
}
