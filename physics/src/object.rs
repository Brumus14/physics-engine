use crate::types::*;

pub enum Shape {
    Point,
    Circle(f64),
    Polygon(Vec<[f64; 2]>),
}

pub struct Object {
    pub position: Vec2<f64>,
    pub velocity: Vec2<f64>,
    pub acceleration: Vec2<f64>,
    // Add unit conversions
    pub mass: f64,
    pub shape: Shape,
}

impl Object {
    pub fn new(position: Vec2<f64>, velocity: Vec2<f64>, mass: f64, shape: Shape) -> Self {
        Self {
            position,
            velocity,
            acceleration: Vec2::zeros(),
            mass,
            shape,
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
    }
}
