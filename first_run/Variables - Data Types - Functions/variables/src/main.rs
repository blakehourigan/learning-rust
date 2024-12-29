fn main() {
    let x = "testing";
    println!("The value of x is: {x}");
    let x = 5 + 1;
    {
        let x = x * 2;
        println!("X is equal to {}", x);
    }
    println!("X is equal to {}", x);
}


