fn main() {
    let a = String::from("hello, world");
    let first_a = first_word(&a);
    println!("first word index: {}", first_a);
    
    let b = String::from("hello, world");
    let first_b = first_word_return_slice(&b);
    println!("first word index: {}", first_b);
    
    string_slice_example();
    string_slices_as_parameters();
    int_slices();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_return_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slice_example() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}", hello);
    println!("{}", world);
}

fn string_slices_as_parameters() {
     let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word_string_slice(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_string_slice(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_string_slice(my_string_literal);
    println!("{}", word);
}

fn first_word_string_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn int_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}