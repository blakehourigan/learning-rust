use std::fs::File;
use std::io::{self, Read};

fn main() {
    let username: Result<String, io::Error> = even_shorter_shorthand();

    println!("{}", username.unwrap());
}

//fn read_username_from_file() -> Result<String, io::Error>{
//    let username_file_result = File::open("hello.txt");
//
//    let mut username_file = match username_file_result {
//        Ok(file) => file, 
//        Err(error) => return Err(error),
//    };
//
//    let mut username = String::new();
//
//    match username_file.read_to_string(&mut username) { 
//        Ok(_) => Ok(username),
//        Err(e) => Err(e),
//    }
//}

//fn propagation_shorthand() -> Result<String, io::Error>{ 
//    let mut username_file = File::open("hello.txt")?;
//    let mut username = String::new();
//    username_file.read_to_string(&mut username)?; 
//    Ok(username)
//}

fn even_shorter_shorthand() -> Result<String, io::Error> {
    let mut username = String::new();
    let _ = File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

