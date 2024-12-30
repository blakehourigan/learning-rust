use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("before declaring the closure {list:?}");

    thread::spawn(move || println!("another thread {list:?}"))
        .join()
        .unwrap();
}
