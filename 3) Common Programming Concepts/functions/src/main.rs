use std::io;

fn main() {
    let mut age = String::new();

    println!("enter your age below:");

    io::stdin().read_line(&mut age).expect("enter something!");

    let age: u32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("enter a positive integer!");
        }
    };

    println!(
        "{0}, you are {1} years old",
        another_function(),
        years_old(age)
    );

    let x = five();

    println!("variable x holds value {x}.");
}

fn five() -> i32 {
    5
}

fn another_function() -> String {
    "hello world!".to_string()
}

fn years_old(years: u32) -> u32 {
    years
}
