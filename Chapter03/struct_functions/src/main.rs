fn main() {
    println!("Hello, world!");
    let c1 = Circle { radius: 1.0 };
    let c2 = Circle { radius: 2.0 };


    println!("Area c1: {}", c1.compute_area());
    println!("Circumference c1: {}", c1.compute_circumference());
    println!("Area c2: {}", c2.compute_area());
    println!{"c1 smaller than c2: {}", c1.smaller(&c2)};
}

struct Circle{
    radius: f32
}

impl Circle{
    fn compute_area(&self) -> f32 {
        (self.radius as f64 * self.radius as f64 * std::f64::consts::PI) as f32
    }
    
    fn compute_circumference(&self) -> f32 {
        (2.0 * self.radius as f64 * std::f64::consts::PI) as f32
    }
    
    fn smaller(&self, other: &Self) -> bool {
        self.radius < other.radius
    }
}