struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    imutable_struct();
    mutable_struct();
    struct_update_syntax();
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

fn struct_update_syntax() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    println!("");
    println!("username: {}", user2.username);
    println!("email: {}", user2.email);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("active: {}", user2.active);
}

fn build_user(email: String, username: String) -> User {
    return User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    };
}
