fn main() {
    iterator_adapters();
}

fn iterator_adapters() {
    // these functions take an iterator and return a new iterator
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();

    println!("{v2:?}");
}

fn basic_iterators() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("{value}");
    }
}
