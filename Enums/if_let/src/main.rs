fn main() {
    let config_max = Some(3u8);
    //match config_max{ <== this code is verbose, can be shortened with the following: 
    //    Some(max) => println!("The maximum is {max}"),
    //    _ => (),
    //}
    
    if let Some(max) = config_max {
        println!("the maximum is: {}", max); // we should only use this when we use a match that
    }                                         // does something if some, and ignores all other
                                             // values
    
}
