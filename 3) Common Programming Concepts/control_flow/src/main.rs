fn main() {
    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
    //
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number was assigned: {number}");

    println!("\nloops!");

    // infinite loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("\nresult of the loop: {result}\n");

    loop_labels();

    while_conditionals();

    liftoff_countdown();

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }
}

fn loop_labels() -> () {
    println!("loop labels");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!();
}

fn while_conditionals() -> () {
    println!("while conditionals");

    let mut num = 3;

    while num != 0 {
        println!("{num}");

        num -= 1;
    }
    println!();
}

fn liftoff_countdown() -> () {
    println!("BEGIN COUNTDOWN");

    for num in (1..4).rev() {
        println!("{num}")
    }

    println!("LIFTOFF");
    println!();
}
