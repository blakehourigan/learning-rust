use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be a number between 1 and 100, got {value}");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your number now: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("This program should compile fine!");

        let guess: i32 = guess.trim().parse().expect("enter a number");

        let guess = Guess::new(guess);

        println!("You guessed the number: {:?}", guess);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("That's right!");
                break;
            }
            Ordering::Greater => println!("Too large..."),
        };
    }
}
