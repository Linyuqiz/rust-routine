use std::f32::consts::PI;

pub(crate) trait Calculate {
    fn area(&self) -> u32;
}

#[derive(Debug)]
pub(crate) struct Round {
    pub(crate) r: u32,
}

impl Calculate for Round {
    fn area(&self) -> u32 {
        PI as u32 * self.r * self.r
    }
}

#[derive(Debug)]
pub(crate) struct Triangle {
    pub(crate) a: u32,
    pub(crate) h: u32,
}

impl Calculate for Triangle {
    fn area(&self) -> u32 {
        self.a * self.h / 2
    }
}

#[derive(Debug)]
pub(crate) struct Square {
    pub(crate) a: u32,
}

impl Calculate for Square {
    fn area(&self) -> u32 {
        self.a * self.a
    }
}

pub(crate) fn calculate_area<T: Calculate>(shape: T) -> u32 {
    shape.area()
}
