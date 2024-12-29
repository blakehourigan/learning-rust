fn main() {
    str_slice_lifetimes();    
}

//fn integer_lifetime() {
//    let r;
//
//
//    let x = 5;
//    r = &x;
//
//
//
//    println!("r is now {r}");
//}

fn str_slice_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result; 

    result = longest(&string1, &string2);

    println!("{result}");

}

fn longest<'a>(x: & 'a str, y: & 'a str) -> & 'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }

}
