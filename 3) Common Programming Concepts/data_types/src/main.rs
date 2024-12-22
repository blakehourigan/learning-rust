use std::io;

fn main() {
    // tuples and arrays!
    // tuples are immutable and you must specify their types...
    let tup: (i32, f64, char) = (500, 69.420, 'c');
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    // arrays!
    // they cannot grow!
    println!("\nnow we are doing arrays!.\n");
    let a = [1, 2, 3, 4, 5]; // this is now fixed at len 5

    let mut index = String::new();

    println!("enter the index of the array that you would like to view!: ");

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("you didn't enter a number bruh");

    let element = a[index];

    println!("the element you chose was: {element} at index {index}");
}
