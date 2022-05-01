use crate::question::TrafficLight;
use crate::question::{calculate_area, Square};
use crate::question::sum;

mod question;

fn main() {
    // question one
    let light = TrafficLight::Green;
    println!("The {:?} light: {}s", light, light.time());

    // question two
    let array = [11, 12, 3, 4, 5, 6, 7];
    if let Some(value) = sum(&array) {
        println!("The value is {}", value)
    } else {
        println!("The value is too big!")
    }

    // question three
    let shape = Square { a: 11 };
    let value = calculate_area(shape);
    println!("The shape's area is {}", value)
}
