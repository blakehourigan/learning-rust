struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer_point = Point { x: 32, y: 32 };

    let float_point = Point { x: 32.22, y: 49.32 };

    let mixed_point = Point { x: 32.22, y: 49 };

    println!("{0} is x", mixed_point.x());

    println!(
        "{0} is the point's distance from the origin.",
        float_point.distance_from_origin()
    );
}
