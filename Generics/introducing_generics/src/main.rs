fn main() {
    let number_list = vec![34, 50, 60, 22, 30];

    let result = largest(&number_list);
    
    println!("largest number in the vector is {result}");

    let char_list = vec!['a','b','c','d','e'];

    let result = largest(&char_list);

    println!("largest char in the char vec is {result}");


}


//fn largest_i32(list: &[i32]) -> &i32 {
//    let mut largest = &list[0];
//
//    for item in list{
//        if item > largest{
//            largest = item; 
//        }
//    }
//    largest 
//}
//
//fn largest_char(list: &[char]) -> &char {
//    let mut largest = &list[0];
//    
//    for item in list {
//        if item > largest{
//            largest = item;
//        }
//    }
//
//    largest
//}

// the above code can be combined using generics into one single function. 
fn largest<T: std::cmp::PartialOrd >(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}

