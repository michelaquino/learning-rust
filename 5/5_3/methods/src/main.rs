fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 30
    };

    println!("width is greater than 0? {}", rectangle.width());
    println!("The area of rectangle {:?} is {}", rectangle, rectangle.area());
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}