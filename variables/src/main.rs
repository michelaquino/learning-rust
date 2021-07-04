fn main() {
    let x = 4; // imutable
    println!("The value of x is: {}", x);
    let mut x = 5; // mutable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("First tuple value: {}", tup.0);

    let (x, y, z) = tup; // This is called destructuring
    println!("Tuple values: {}, {}, {}", x, y, z);
    
    // Array
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Tuple values: {}", array[0]);
    
    let array_same_element = [5; 3];    
    println!("Tuple values: {}, {}, {}", array_same_element[0], array_same_element[1], array_same_element[2]);
}
