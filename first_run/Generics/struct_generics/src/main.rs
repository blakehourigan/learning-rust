struct Point<T> {
    x: T,
    y: T, 
}

struct Point_2<T, U> { // this allows us to use 2 different generics
    x: T,
    y: U,
}

fn main() {
    let integer = Point{x: 5, y: 10};   
    let float = Point{x: 5.43, y: 10.20};

    println!("the point using integers in (x, y) is: ({0},{1})",integer.x ,integer.y);
    println!("the point using floats in (x, y) is: ({0}, {1})", float.x, float.y);

    let int_float = Point_2{x: 2, y: 5.2};
    let float_int = Point_2{x: 2.3, y: 5};
    println!("the point using integers in (x, y) is: ({0},{1})",int_float.x ,int_float.y);
    println!("the point using floats in (x, y) is: ({0}, {1})", float_int.x, float_int.y);
}
