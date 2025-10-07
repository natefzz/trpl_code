use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    //----------------------------------------------------------

    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt"); // better error message

    read_username_from_file().expect("Failed to read username from file");
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hellccco.txt")
}
