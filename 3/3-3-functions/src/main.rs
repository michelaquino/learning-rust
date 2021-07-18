fn main() {
    another_function(5, 6);
    expression();
    
    let five = five();
    println!("five: {}", five)
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("Values -> x is {}; y is {}", x, y);
}

fn five() -> i32 {
    5
}