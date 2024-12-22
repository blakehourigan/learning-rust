#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // let rectangle = (30, 50);

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    dbg!(rectangle.width * rectangle.height)
}
