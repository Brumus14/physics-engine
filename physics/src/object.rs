use crate::types::math::*;

pub enum Shape {
    Point,
    Circle(f64),
    Polygon(Vec<[f64; 2]>),
}

pub struct Object {
    pub position: Vector<f64>,
    pub velocity: Vector<f64>,
    pub force: Vector<f64>,
    pub mass: f64,
}

impl Object {
    pub fn new(position: Vector<f64>, velocity: Vector<f64>, mass: f64) -> Self {
        Self {
            position,
            velocity,
            force: Vector::zeros(),
            mass,
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        self.velocity += (self.force / self.mass) * delta_time;
        self.position += self.velocity * delta_time;
        self.force = Vector::zeros();
    }
}
