fn main() {
    references();
}

fn references(){
    let mut s1 = String::from("hello, world!");

    let len = calculate_length(&mut s1);

    println!("the length of the string is {len}");
    println!("the string is: {s1}");
}

fn calculate_length(s: &mut String) -> usize{
    s.push_str("hello");
    s.len() 
}
