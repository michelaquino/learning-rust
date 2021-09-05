fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    
    let mut s3 = String::from("hello");
    let len_s3 = calculate_length_reference(&s3);
    println!("The length of '{}' is {}.", s3, len_s3);
    
    change(&mut s3);
    println!("modified '{}'", s3);

} 

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
  s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
