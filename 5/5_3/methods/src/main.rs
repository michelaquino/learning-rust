fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 30
    };
    
    let rectangle2 = Rectangle {
        width: 10,
        height: 10
    };

    println!("width is greater than 0? {}", rectangle1.width());
    println!("The area of rectangle {:?} is {}", rectangle1, rectangle1.area());
    println!("Rectangle {:?} can hold rectangle {:?}? {}", rectangle1, rectangle2, rectangle1.can_hold(&rectangle2));
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
    
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.area() > another.area()
    }
}