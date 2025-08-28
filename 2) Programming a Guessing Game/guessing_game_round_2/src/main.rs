use std::cmp::Ordering;

use std::io;

use rand::Rng;

fn main() {
    println!("guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("its supposed to be a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a NUMBER");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("higher!"),
            Ordering::Equal => {
                println!("thats it!");
                break;
            }
            Ordering::Greater => println!("lower!"),
        }

        println!("You guessed: {guess}");
    }
}
