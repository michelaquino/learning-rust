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