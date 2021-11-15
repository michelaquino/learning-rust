fn main() {
    // vectors();
    // vector_store_multiple_types();
    strings();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    println!("{:?}", v);
    
    // iterating
    for i in &v {
        println!("{}", i);
    }
    
    // Iterate on a mutable reference
    for i in &mut v {
        *i *= 10;
        println!("{}", i);
    }

    // Another way to create vector
    let mut v1 = vec![1,2,3];
    println!("{:?}", v1);

    // Selecting elements
    println!("first element: {}", v1[0]);
    match v1.get(3) {
        Some(fourth) => println!("fourth element: {}", fourth),
        None => println!("fourth element not found"),
    }
    
    // change a element
    v1[0] = 10;
    println!("first element changed: {}", v1[0]);
}

fn vector_store_multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("ROW: {:?}", row);
}

fn strings() {
    let mut new_string = String::new();
    println!("Value: {}", new_string);
    
    let data = "content";
    println!("Value: {}", data);
    
    let s = data.to_string();
    println!("Value: {}", s);
    
    let s2 = String::from("another content");
    println!("Value: {}", s2);
    
    // UTF-8 encoded
    let hello = String::from("你好");
    println!("Value: {}", hello);
    
    
    /////////// Updating string
    let mut updating = String::from("foo");
    updating.push(' ');
    updating.push_str("bar");
    println!("updating Value: {}", updating);
    
    
    let s1 = String::from("hello");
    println!("s1: {}", s1);
    let s2 = String::from("world");
    println!("s2: {}", s2);
    
    let s4 = format!("{} {}", s1, s2);
    println!("s4: {}", s4);
    
    let s3 = s1 + &s2;
    println!("s3: {}", s3);


    let hello = "Здравствуйте";
    let s = &hello[0..4]; // each char on string has 2 bytes. Here we are get 4 bytes, 2 letters
    println!("s: {}", s);

    println!("Iterating on string, chars:");
    for c in hello.chars() {
        println!("{}", c);
    }
    
    println!("Iterating on string, bytes:");
    for c in hello.bytes() {
        println!("{}", c);
    }
}