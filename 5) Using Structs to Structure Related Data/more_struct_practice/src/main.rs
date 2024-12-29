struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, another_rectangle: &Self) -> bool {
        if self.area() >= another_rectangle.area() {
            return true;
        } else {
            return false;
        }
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle::square(50);

    let rectangle3 = Rectangle {
        width: 5,
        height: 10,
    };

    println!(
        "rect 1 is larger than rect 2: {0}",
        rectangle1.can_hold(&rectangle2)
    );

    println!(
        "rect 2 is can hold rectangle 3: {0}",
        rectangle2.can_hold(&rectangle3)
    );

    println!(
        "rect 1 is can hold rectangle 3: {0}",
        rectangle1.can_hold(&rectangle3)
    );
}
