fn main() {
    println!("The value of {:?} is {}", Coin::Penny, value_in_cents(&Coin::Penny));
    println!("The value of {:?} is {}", Coin::Nickel, value_in_cents(&Coin::Nickel));
    println!("The value of {:?} is {}", Coin::Dime, value_in_cents(&Coin::Dime));
    println!("The value of {:?} is {}", Coin::Quarter, value_in_cents(&Coin::Quarter));
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => return 1,
        Coin::Nickel => {
            return 5
        },
        Coin::Dime => {
            println!("Dime matched");
            10
        },
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}