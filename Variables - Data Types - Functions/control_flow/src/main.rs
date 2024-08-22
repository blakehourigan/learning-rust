fn main() {
   looping_thru_collections();
}

//fn if_expressions(){
//    let number = 7; 
//
//    if number < 5{
//        println!("The number is less than 5");
//    }else{
//        println!("The number is greater than five");
//    }
//}

//fn condition_must_be_bool(){
//    let number = 3;
//
//    if number != 0 {
//        println!("number is not equal to zero");
//    }
//}
//
//fn multiple_ifs(){
//    let number = 6; 
//
//    if number % 4 == 0 {
//        println!("number is divisible by 4!");
//    }
//    else if number % 3 == 0 {
//        println!("number is divisible by 3!");
//    }
//    else if number % 2 == 0 {
//        println!("number is divisible by 2!");
//    }
//    else {
//        println!("number is not divisible by either 4, 3, or 2");
//    }
//}

//fn if_in_a_let(){
//    let condition = true;
//    let number = if condition {5} else {6};
//
//    println!("The number is {number}");
//}

//fn loops(){
//    let mut counter = 0 ; 
//
//    let result = loop{
//        counter += 1;
//        
//        println!("{counter}");
//
//        if counter == 10{
//            break counter * 2; 
//        }
//    };
//    println!("resulting value from loop is {result}");
//}

//fn loop_labels(){
//    let mut count = 0;
//    'counting_up: loop {
//        println!("count = {count}");
//        let mut remaining = 10; 
//
//        loop{
//            println!("remaining = {remaining}");
//            if remaining == 9{
//                break;
//            }
//            if count == 2{
//                break 'counting_up;
//            }
//            remaining -= 1; 
//        }
//        count += 1;
//    }
//    println!("final counter value: {count}");
//}

//fn while_conditionals(){
//    let mut number = 3; 
//
//    while number != 0 {
//        print!("{number}...");
//        number -= 1;
//    }
//    println!(" LIFTOFF!!!");
//}

fn looping_thru_collections(){
    let a = [10, 20, 30, 40, 50];
    
    for element in a{
        println!("{element}");
    }
}

