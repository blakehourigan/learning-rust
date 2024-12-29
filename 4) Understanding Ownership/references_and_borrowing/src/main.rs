fn main() {
    let mut s1 = String::from("hello");
    let r1 = &mut s1;
    // let len = calculate_length(&s1);
    change(r1);
    println!("{s1} ");
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn change(s: &mut String) {
    s.push_str(", world!");
}
