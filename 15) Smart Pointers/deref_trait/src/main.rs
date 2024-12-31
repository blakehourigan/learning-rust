use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    mybox_deref();
}

fn mybox_deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(x, *y);
}

fn basic_deref() {
    // implementing the deref trait allows you to customize the behavior
    // of the deref operator: '*'

    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(x, *y);
}

fn box_deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(x, *y);
}
