use std::io;

fn main() {
    println!("welcome to the fahrenheit -> celsius conversion tool!");

    loop {
        println!("enter the number you want to convert!");
        let mut fahrenheit_num = String::new();

        io::stdin()
            .read_line(&mut fahrenheit_num)
            .expect("error accepting input");

        let fahrenheit_num: f32 = match fahrenheit_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a integer or floating point number");
                continue;
            }
        };

        let celsius_number = (fahrenheit_num - 32.0) * (5.0 / 9.0);

        println!("the temperature of {fahrenheit_num} in celsius is {celsius_number}");
    }
}
