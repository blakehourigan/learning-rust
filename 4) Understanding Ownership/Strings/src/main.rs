fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{s}");

    // differences between stack vars and heap vars

    // let x = 5;
    // let y = x;

    // println!("we see that both x and y are 5, x={x}, y={y}");

    // however, doing similar things with Strings...

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s2}");

    let s = String::from("hello");

    takes_ownership(s);

    // does not work: println!("{s}");

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("took ownership of {some_string}");
}

fn makes_copy(some_number: u32) {
    println!("made a copy of {some_number}");
}
