fn main() {
    let version4 = IpAddrKind::V4;
    let version6 = IpAddrKind::V6;

    println!("Vertsion 4: {:?}", version4);
    println!("Vertsion 6: {:?}", version6);
    
    let localhost = IpAddress{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddress{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("localhost: {:?}", localhost);
    println!("loopback: {:?}", loopback);

    println!("localhost: {:?}", IpKind::V4(String::from("127.0.0.1")));
    println!("loopback: {:?}", IpKind::V6(String::from("::1")));
   
    println!("[constructor] localhost: {:?}", IpKindConstructor::V4(127, 0, 0, 1));
    println!("[constructor] loopback: {:?}", IpKindConstructor::V6(String::from("::1")));

    println!("This is the message Message::Quit called: {:?}", Message::Quit.call());
    println!("This is the message Message::Move called: {:?}", Message::Move{x: 1, y: 2}.call());
    println!("This is the message Message::Write called: {:?}", Message::Write(String::from("A message")).call());
    println!("This is the message Message::ChangeColor called: {:?}", Message::ChangeColor(1, 2, 3 ).call());
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpKindConstructor {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("The enum was called")
    }
}