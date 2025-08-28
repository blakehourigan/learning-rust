// in this program we use binet's formula to calculate the nth fibonacci number

use std::io;

fn main() {
    println!(
        "size of isize on my machine in bytes: {0}, size of usize: {1}",
        size_of::<isize>(),
        size_of::<usize>()
    );

    let sqrt_5 = (5 as f64).sqrt();
    let phi: f64 = (1.0 + sqrt_5) / 2.0;
    let psi: f64 = (1.0 - sqrt_5) / 2.0;

    println!("enter the nth number you would like the fibonacci of:");

    loop {
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("error reading the std in");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a positive integer!");
                continue;
            }
        };

        let phi = phi.powf(n as f64);
        let psi = psi.powf(n as f64);

        let nth_term: u64 = ((phi - psi) / sqrt_5) as u64;

        println!("the {n} term of the fib sequence is {nth_term}");
    }
}
