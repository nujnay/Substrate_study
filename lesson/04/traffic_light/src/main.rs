fn main() {
    println!("Hello, world!");
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 50,
            TrafficLight::Yellow => 10,
            _ => {}
        }
    }
}