fn main() {
    indexing_into_strings();
}

//fn concatentating_strings() {
//    let s1 = String::from("tic");
//    let s2 = String::from("tac");
//    let s3 = String::from("toe");                                                             }
//
//    let formatted_string = format!("{s1}-{s2}-{s3}");
//
//    println!("{formatted_string}");
//
//    // the above is much cleaner than the alternative uing add (+)
//    
//    let formatted_string_alternative = s1 + "-" + &s2 + "-" + &s3 ;
//
//    println!("{formatted_string_alternative}");
//
//    // the outputs are exactly the same but the second one is a little harder to use and read.
//}

fn indexing_into_strings() {
    let s1 = String::from("hello");
    
    //let h = s1[0]; // this doesn't work for a multitude of reasons, namely that rust doesn't really
                   // know what you want when you say this, and because rust uses utf-8 encoding,
                   // chars can be 1-4 bytes long. So to do this we shuold instead specify that we
                   // want chars or bytes. 
    for c in s1.chars(){
        println!("{c}"); 
    }

    // you can also specify that you want to print the byte values for each entry by saying the
    // following 
    
    for b in s1.bytes() {
        println!("{b}"); 
    }
}
