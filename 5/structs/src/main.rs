struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    imutable_struct();
    mutable_struct();
}

fn mutable_struct() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    println!("email: {}", user1.email);
    user1.email = String::from("another@email.com");
    println!("email: {}", user1.email);
}

fn imutable_struct() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!("active: {}", user1.active);
}

fn build_user(email: String, username: String) -> User {
    return User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    };
}