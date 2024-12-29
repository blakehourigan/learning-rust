fn main() {
    let mut s = String::new();

    let data = "string contents";

    let data: String = data.to_string();
    println!("{data}");

    let mut s1 = String::from("foo");

    let s2 = "bar";

    s1.push_str(s2);

    // the above can also be performed by doing the following:
    // s3 = s1 + &s2

    println!("{s2}");

    let s3 = format!("{s1}-{s2}");

    println!("{s3}");
}
