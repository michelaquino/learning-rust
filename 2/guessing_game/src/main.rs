use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    get_user_guess()
}

fn get_user_guess() {
    loop {
        println!("Please enter you guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        
        println!("You guess: {}", guess);
    
        let secret_number = rand::thread_rng().gen_range(1..11);
        let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(error) => {
                    println!("Error: '{}'", error);
                    continue;
                }
            };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}