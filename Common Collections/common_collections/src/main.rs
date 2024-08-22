fn main() {
    let mut v: Vec<i32> = Vec::new();

    let mut v1 = vec![1, 2, 3];
    
    v.push(69);

//    println!("{:?}", v1.get(0));// this way of accessing it gets you an Option<T> value at 0
//    println!("{:?}", &v1[0]);   // this way of doing things gets you the actual value at 0
//    
//    println!(" "); 
//
//    println!("{:?}", v.get(0));
//    println!("{:?}", &v[0]); // accessing array elements this way can result in out of bounds error  

    // println!("{:?}", &v[34]); this causes error because len is 1 but 34 is being accessed
   

    for i in & mut v1 {
        *i = *i + 1;  
    }

    for i in &mut v1 {
        println!("{:?}", *i);
    }


}
