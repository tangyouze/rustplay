use core::f64::consts::PI;
fn main() {
    let v = Circle { radius: 1.2 };
    println!("circle area {}", v.area());

    let sq = Square { width: 2.4 };
    println!("square area {}", sq.area());
}

pub trait Area {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub width: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        return self.width * self.width;
    }
}
