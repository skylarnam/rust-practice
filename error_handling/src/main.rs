use std::fs::File;
use std::{fs, io};
use std::error::Error;
use std::io::{ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // let greeting_file_result = File::open("hello.txt");
    //
    // let _greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
    //     ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating the file: {e:?}"),
    //     },
    //     other_error => {
    //         panic!("Problem opening the file: {other_error:?}");
    //     }
    // });

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}