use std::io;

fn main() {
// tuple_example();
invalid_array_index_access_ex()
}

fn tuple_example(){
    let tup = (12,36.121,23);

    let x = tup.0;

    let y = tup.1; 

    let z = tup.2;

    println!("the value of x is {x}, the value of y is {y}, and the value of z is {z}");
}

fn invalid_array_index_access_ex(){
   let a = [1, 2, 3, 4, 5]; 

   println!("please enter an array index: ");

   let mut index = String::new();
   
   io::stdin().read_line(&mut index).expect("unable to read line");

   let index: usize = index.trim().parse().expect("please make sure to enter an integer.");

   let element = a[index];

   println!("the element is: {element}");
}
