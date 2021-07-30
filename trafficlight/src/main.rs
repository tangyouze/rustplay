fn main() {
    let t = TrafficLight::Green;
    println!("wait {}", t.wait());

    let t = TrafficLight::Red;
    println!("wait {}", t.wait());
}

enum TrafficLight {
    Green,
    Yellow,
    Red,
}

trait Light {
    fn wait(&self) -> f64 {
        return 10.2;
    }
}

impl Light for TrafficLight {
    fn wait(&self) -> f64 {
        match self {
            TrafficLight::Green => {
                return 13.1;
            }
            TrafficLight::Red => {
                return 99.1;
            }
            TrafficLight::Yellow => {
                return 3.1;
            }
        }
        return 20.3;
    }
}
