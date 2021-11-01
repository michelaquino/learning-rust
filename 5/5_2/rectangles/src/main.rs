fn main() {
    let widht = 30;
    let height = 50;
    let dimensions = (30, 50);

    println!("The area of retangle is {}", area(widht, height));
    println!("[Tuple] The area of retangle is {}", area_tuple(dimensions));
}

fn area(widht: i32, height: i32) -> i32 {
    return widht * height
}

fn area_tuple(dimensions: (i32, i32)) -> i32 {
    return dimensions.0 * dimensions.1
}