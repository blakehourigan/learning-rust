use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("ensure the guess is within the bound of 1 and 100 (inclusive).");
        }
        Guess{value}
    }
    pub fn value(&self) -> i32{
        self.value
    }
}

fn main() {
    println!("welcome to el guessing gamo");
    
    loop{
        let mut rng = thread_rng();
        let lucky_number = rng.gen_range(1..=10); 

        println!("guess a number between 1 and 100");

        let mut guess_value = String::new();

        let _ = io::stdin().read_line(&mut guess_value).expect("enter a string");

        let guess_value:i32 = match guess_value.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess_value);
       
        match Guess::value(&guess).cmp(&lucky_number) {
            Ordering::Less => println!("your guess was too low"),
            Ordering::Equal => {
                println!("your guess was exactly right");
                break },
            Ordering::Greater => println!("your guess was too high"),
        };
        print_lucky_number(&lucky_number);
    }
}

fn print_lucky_number(x: &i32){
    println!("the lucky number was: {x}");
}
