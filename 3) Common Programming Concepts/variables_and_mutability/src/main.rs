fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("the amount of seconds in 3 hours is: {THREE_HOURS_IN_SECONDS}.");

    let x = 5;
    println!("The value of x is {x}.");
    let mut x = x + 1;
    println!("The value of x is {x}.");
    x = 7;
    println!("The value of x is {x}.");

    // you can also shadow without making the new variable mutable...
    println!("\n\n");
    let y = 5;
    println!("The value of y is {y}.");
    let y = y + 1;
    println!("The value of y is {y}.");
}
