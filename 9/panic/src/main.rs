// Important note: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic

fn main() {
    // panic_application();
    panic_from_another_source();

}

fn panic_application() {
    panic!("this panic the application");
}

fn panic_from_another_source() {
    let vector = vec![1,2,3];

    println!("{}", vector[99])
}