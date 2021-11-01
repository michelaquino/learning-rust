fn main() {
    let widht = 30;
    let height = 50;

    println!("The area of retangle is {}", area(widht, height));
}

fn area(widht: i32, height: i32) -> i32 {
    return widht * height
}