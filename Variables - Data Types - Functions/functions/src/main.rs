fn main() {
    let x = plus_one(2147483646);

    println!("the value that you gave me is: {x}");
}

fn print_labelled_measurements(x: i32, unit_label: char){
    println!("The measurement is: {x}{unit_label}");
}

fn statements_and_expressions(){
    let y = {      // this is a statement, the opening curlies starts a new 'scope' which is an
                   // expression. 
        let x = 3; // this is a statement
        x + 1      // this is an expression, so the overall expression will evaluate to the result
                   // of this expression. 
    };
    println!("the value of the overall expression is: {y}");
}

fn five() -> i32{
   32 
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
