fn main() {
    mutable_imutable();
    
    // // Tuples
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("First tuple value: {}", tup.0);

    // let (x, y, z) = tup; // This is called destructuring
    // println!("Tuple values: {}, {}, {}", x, y, z);
    
    // // Array
    // let array: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("Tuple values: {}", array[0]);
    
    // let array_same_element = [5; 3];    
    // println!("Tuple values: {}, {}, {}", array_same_element[0], array_same_element[1], array_same_element[2]);
}


fn mutable_imutable() {
    let x = 4; // imutavel
    println!("{}", x);

    let mut y = 5; // mutavel
    println!("{}", y);
    y = 6;
    println!("{}", y);

    let z = 7; // imutavel
    println!("{}", z);
    let z = 8; // imutavel
    println!("{}", z);
}