fn main() {
    let widht = 30;
    let height = 50;
    let dimensions = (30, 50);

    let rectangle = Rectangle{
        width: 30,
        height: 50,
    };

    dbg!(&rectangle);

    println!("The area of rectangle is {}", area(widht, height));
    println!("[Tuple] The area of rectangle is {}", area_tuple(dimensions));
    println!("[Struct] The area of rectangle {:?} is {}", rectangle, area_struct(&rectangle));
}

fn area(widht: i32, height: i32) -> i32 {
    return widht * height
}

fn area_tuple(dimensions: (i32, i32)) -> i32 {
    return dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

// & -> borrow
fn area_struct(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}