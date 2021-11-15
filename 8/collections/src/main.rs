fn main() {
    vectors();
    vector_store_multiple_types();
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