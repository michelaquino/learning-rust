use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // handle_result(); 
    // handle_result_with_different_errors();
    // using_unwarap();
    using_expected();
    propagating_errors();
}

fn using_unwarap() {
    let f = File::open("file.txt").unwrap();
}

fn using_expected() {
    let f = File::open("file.txt").expect("customized panic message");
}

fn handle_result() {
    let f = File::open("file.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn handle_result_with_different_errors() {
    let file_path = "file.txt";
    let f = File::open(file_path);

    let file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file_created) => file_created,
                Err(error) => panic!("Error when create the file {}: {}", file_path, error)
            }
            _unknonwn_error => panic!("Problem opening the file: {:?}", error),
        }
    };
}

fn propagating_errors() -> Result<String, io::Error>{
    let file_path = "file.txt";
    let f = File::open(file_path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => return Ok(s),
        Err(err) => return Err(err),
    }
}

/*
The ? at the end of the File::open call will return the value inside an Ok to the variable f. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.
*/
fn propagating_errors_interrogation_operator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn propagating_errors_interrogation_operator_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn propagating_errors_interrogation_operator_even_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}