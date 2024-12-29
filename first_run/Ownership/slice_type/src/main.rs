fn main() {
    let s1 = String::from("no spaces is the string");
    let s2 = first_word(&s1);
    println!("{s2}");
//    testing_slice_things();
//    clarifying_slices();
}

fn first_word(s: &str) -> &str{
    let mut index = 0;
    for c in s.chars(){
        if c == ' '{
            let s0 = &s[0..index]; 
            return s0
        }
        else if index == (s.len() - 1){
            println!("no spaces in the string");
            let y = s.len();
            return &s[0..y]; 
        }
        index += 1;
    }
    "none"
}

fn second_word(s: &str) -> &str{
    let mut index = 0; 

}

//fn testing_slice_things() -> String{
//    let s = String::from("hello world");
//
//    let hello = String::from(&s[..5]);
//    let world = String::from(&s[6..]);
//
//    println!("{hello}, {world}");
//
//    hello
//}

//fn clarifying_slices(){
//    let s = String::from("hello world");
//    
//    let s1 = String::from(&s[..2]);
//    let s2 = &s[..2];
//
//    print_type_of(&s1);
//    print_type_of(&s2);
//    
//}
//
//fn print_type_of<T>(_: &T){
//    println!("{}", std::any::type_name::<T>());
//}
