pub enum Shape {
    Reactange(f64, f64), // we use patter matching to access these values
    Circle(f64),
}

pub fn calc_area(shape: Shape) -> f64 {
    match shape {
        // this is kinda like a function
        Shape::Reactange(a, b) => {
            let area = a * b;
            return area + 1.0;
        }
        Shape::Circle(r) => 3.14 * r * r,
    }
}
