fn main() {
    let s = String::from("the quick brown fox jumped over the lazy dog");

    let s1 = first_word(&s);
    println!("{s1}");
}

fn first_word(some_string: &str) -> &str {
    //     let a = some_string.split(" ");
    //     for word in a {
    //         if word != "" {
    //             return word;
    //     }
    //     some_string
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }
    some_string
}
