use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world"; 

    let mut map = HashMap::new();
    
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0); // count is a mutable reference because that is
        *count += 1;                              // how or_insert is designed 
    }

    println!("{map:?}");
}

//fn introducing_hash_maps(){
//    let mut scores = HashMap::new(); // by default, hash maps use SipHash
//
//    scores.insert(String::from("Blue"), 10);
//    scores.insert(String::from("Yellow"), 50);
//
//    scores.entry(String::from("Yellow")).or_insert(25); // if the value exists, leave it, if not
//                                                        // insert the new given value
//
//    scores.entry(String::from("Green")).or_insert(75);
//
//
//    let score = scores.get(& String::from("Blue")).copied().unwrap_or(0);
//
//    println!("{:?}", score);
//
//    for (key, value) in &scores {
//        println!("Key is {key}, Value is {value}"); 
//    }
//}
