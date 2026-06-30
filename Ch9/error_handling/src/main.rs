use core::error;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    while let Some(c) = last_char_of_file_lines(&data) {
        println!("Read char {}", c);
    }
}

fn last_char_of_file_lines(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open("Hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }
