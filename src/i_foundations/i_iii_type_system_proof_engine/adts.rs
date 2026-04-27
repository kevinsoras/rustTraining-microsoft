// C# — Discriminated unions require sealed-class boilerplate.
// The compiler warns about missing cases (CS8524) ONLY when there's no _ catch-all.
// In practice, most C# code uses _ as a default, which silences the warning.
/* public abstract record Shape;
public sealed record Circle(double Radius)   : Shape;
public sealed record Rectangle(double W, double H) : Shape;
public sealed record Triangle(double A, double B, double C) : Shape;

public static double Area(Shape shape) => shape switch
{
    Circle c    => Math.PI * c.Radius * c.Radius,
    Rectangle r => r.W * r.H,
    // Forgot Triangle? The _ catch-all silences any compiler warning.
    _           => throw new ArgumentException("Unknown shape")
}; */
// Add a new variant six months later — the _ pattern hides the missing case.
// No compiler warning tells you about the 47 switch expressions you need to update.

// Rust — Enums with pattern matching ensure all cases are handled.
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { w: f64, h: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}
pub fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { w, h } => w * h,
        Shape::Triangle { a, b, c } => {
            //Semiperimeter
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}
