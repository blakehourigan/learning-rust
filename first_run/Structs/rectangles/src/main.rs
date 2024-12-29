#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle { 
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn width(&self) -> bool { 
        self.width > 0
    }
    fn can_hold(&self, rectangle2: &Self) -> &str{ 
        if (self.length * self.width) > (rectangle2.length * rectangle2.width){
            "yes"
        }else{
            "no"
        }
    }
    fn square(size: u32) -> Self{
        Self{
            length: size,
            width: size,
        }
    }
}

fn main() {
    a_better_way_w_structs();
}

//fn standard_method(){
//    let length = 30;
//    let width = 50; 
//
//    let result = area(length, width);
//
//    println!("{result}");
//}
//
//fn area(length: i32, width: i32) -> i32{
//    length * width
//}

//fn tuples(){
//   let rect1 = (30,50);
//
//   println!("The area is {}",
//       tuple_area(rect1));
//}
//
//fn tuple_area(dimensions: (i32, i32)) -> i32 {
//    dimensions.0 * dimensions.1
//}

fn a_better_way_w_structs(){
    let rect1 = Rectangle {
        length: 50,
        width: 30, 
    };
    let rect2 = Rectangle{
        length: 60,
        width: 30,
    };
    let rect3 = Rectangle{
        length: 10,
        width: 5,
    };
    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("can rect2 hold rect3: {}", rect2.can_hold(&rect3));

    println!("The area of the rectangle is: {} units.",
        rect1.area());
    let rect4 = Rectangle::square(32);

    println!("Rectangle 4 is {rect4:#?}");
}

//fn struct_area(rectangle: &Rectangle) -> u32 { this function was needed prior to implementing 
//    rectangle.length * rectangle.width         the functionality as a struct method above
//}
