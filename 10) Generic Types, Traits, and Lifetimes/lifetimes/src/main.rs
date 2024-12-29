fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        // string literals have a static lifetime...
        // this means that they are valid for the entirety of
        // the program and that this will not be a problem for
        // the 'longest' function which uses a reference to it...
        let string2 = "xyz";

        result = longest(string1.as_str(), string2);
    }
    println!("the longest string is {}", result);
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}
