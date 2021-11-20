use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // handle_result(); 
    // handle_result_with_different_errors();
    // using_unwarap();
    using_expected();
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