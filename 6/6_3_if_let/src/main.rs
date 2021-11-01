fn main() {
    example_with_match();
    example_with_let();
}


fn example_with_match() {
    let config_max = Some(3);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

fn example_with_let() {
    let config_max = Some(3);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}