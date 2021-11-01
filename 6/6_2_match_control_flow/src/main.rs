fn main() {
    println!("The value of {:?} is {}", Coin::Penny, value_in_cents(&Coin::Penny));
    println!("The value of {:?} is {}", Coin::Nickel, value_in_cents(&Coin::Nickel));
    println!("The value of {:?} is {}", Coin::Dime, value_in_cents(&Coin::Dime));
    println!("The value of {:?} is {}", Coin::Quarter(UsState::Alabama), value_in_cents(&Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    println!("Five: {:?}", five);
    let six = plus_one(five);
    println!("Six: {:?}", six);
    let none = plus_one(None);
    println!("None: {:?}", none);
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
        Coin::Quarter(state) => {
            match state {
                UsState::Alabama => {
                    println!("Alabama");
                }
                UsState::Alaska => {
                    println!("Alaska");
                }
            }
            
            return 25
        },
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
