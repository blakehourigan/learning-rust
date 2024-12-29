fn main() {
    ownership_and_functions();
}

//fn scope(){
//    {
//        let mut s = String::from("hello");
//
//        s.push_str(", world!");
//
//        println!("{s}");
//    }
//}

//fn interaction_with_same_data(){
//    let x = 5;        this code results in an x and a y that are both 5. 
//    let y = x;
//
//    println!("x: {x}, y: {y}");
//
//    let s1 = String::from("hello"); this is considered a 'move'. s1 is invalidated after
//                                    assigning s2 to the value of s1 to prevent double freeing the
//                                    memory in the heap when the variables go out of scope.  
//    let s2 = s1;
//
//    println!("{s2}");
//
//
//    let s1 = String::from("hello");
//    let s2 = s1.clone();
//
//    println!("{s1}, {s2}");
//}

fn ownership_and_functions(){
    let s = String::from("hello");
    
    takes_ownership(s);

    let x = 5; 

    makes_copy(x);

    let given_str = gives_ownership();

    println!("{x}, given ownership of: {given_str}");
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_number: i32){
    println!("{some_number}");
}

fn gives_ownership() -> String{
    let s2 = String::from("world");
    s2
}
