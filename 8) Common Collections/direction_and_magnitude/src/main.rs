fn main() {
    let mut v1 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);

    let _: Option<i32> = match v1.get(2) {
        Some(num) => {
            println!("third is {num}");
            Some(*num)
        }
        None => None,
    };

    for i in v1 {
        println!("{i}");
    }
}
