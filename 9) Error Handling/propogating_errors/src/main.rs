use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // instead of handling errors in the context that you are currently in, you can instead
    // propagate that error back to the calling code, which may have better error handling
    // code or logic.

    let username = shortest_code();

    println!("{0}", username.unwrap());
    Ok(())
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(error) => Err(error),
//     }

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn even_shorter_code() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn shortest_code() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
