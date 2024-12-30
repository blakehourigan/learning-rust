fn main() {
    //capturing_references();
    another_example();
}

fn type_annotation_inference() {
    let closure = |x| x;

    let s = closure(String::from("hello"));

    // let n = closure(5); this does not work, because the first usage of the
    // closure locks in type string for x and therefore the return value as well.
}

fn capturing_references() {
    let mut list = vec![1, 2, 3];
    println!("before defining the closure: {list:?}");

    // let only_borrows = || println!("from closure {list:?}");
    // the above works because we can have as many immutable references to
    // a variable as we would like

    let mut mutable_borrow = || list.push(7);

    mutable_borrow();
    println!("after calling the closure: {list:?}");
}

// super weird on first glance, however, this works because you are giving ownership
// of 'func' away to the sort function which then has the ability to redefine func as
// mutable as needed
fn another_example() {
    let mut num_ops = 0;
    let func = |r: &i32| {
        num_ops += 1;
        r.abs()
    };
    let mut list = [-3, 1, 5];
    list.sort_by_key(func);
    println!("{:?}, {}", list, num_ops);
}
