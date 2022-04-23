#[derive(Debug)]
pub(crate) enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    pub(crate) fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 15,
        }
    }
}
