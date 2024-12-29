fn main() {
    let five = Some(5);
    let none: Option<i32> = None;

    println!(
        "five var equals {0:?}, none var equals {1:?}",
        plus_one(five),
        plus_one(none)
    );
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
