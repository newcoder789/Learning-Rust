enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
    Square(f64)
}

fn main() {
    let circle =  Shape::Circle(7.0);
    let square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);

    println!( "Circle area: {} ", calculate_area(circle));
    println!("Square area: {} ", calculate_area(square));
    println!("Rectangle area: {}", calculate_area(rectangle));
}

fn calculate_area(shape: Shape) -> f64 {
    match shape{
        Shape::Rectangle(a, b)=>a* b,
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(a) => a*a
    }
}