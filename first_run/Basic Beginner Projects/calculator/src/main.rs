use std::io;


fn main() {
    println!("Calculator 0.1");

    println!("1) Addition");
    println!("2) Subtraction");
    println!("3) Multiplication");
    println!("4) Division");
    
    println!("Please select the mode you would like to use from the menu above");
    
    let mut selection = String::new();
    
    io::stdin().read_line(&mut selection).expect("Failed to read line");
    
    let selection: u32 = selection.trim().parse().expect("Please input a number");
    
    println!("Enter the first of the numbers you would like to perform this operation on: ");

    let mut num1 = String::new();

    io::stdin().read_line(&mut num1).expect("unable to read line");

    let num1 : u32 = num1.trim().parse().expect("make sure you enter a number");

    println!("Enter the first of the numbers you would like to perform this operation on: ");
   
    let mut num2 = String::new();

    io::stdin().read_line(&mut num2).expect("unable to read line");

    let num2 : u32 = num2.trim().parse().expect("make sure you enter a number");

    match selection{
        1 => println!("{}", addition(num1, num2)),
        2 => println!("{}", subtraction(num1, num2)),
        3 => println!("{}", multiplication(num1, num2)),
        4 => println!("{}", division(num1, num2)),
        _ => println!("Please enter a valid selection and try again."), 
    }                                                           
}

fn addition(num1: u32,  num2: u32) -> u32{
    num1 + num2 
}
fn subtraction(num1: u32,  num2: u32) -> u32{
    num1 - num2
}
fn multiplication(num1: u32,  num2: u32) -> u32{
    num1 * num2
}
fn division(num1: u32,  num2: u32) -> u32{
    num1 / num2
}

