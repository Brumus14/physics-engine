use crate::types::*;

pub enum Shape {
    Point,
    Circle(f64),
    Polygon(Vec<[f64; 2]>),
}

pub struct Object {
    pub position: Vec2<unit::f64::Length>,
    pub velocity: Vec2<unit::f64::Velocity>,
    pub acceleration: Vec2<unit::f64::Acceleration>,
    // Add unit conversions
    pub mass: unit::mass::kilogram,
    pub shape: Shape,
}

impl Object {
    pub fn new(
        position: Vec2<unit::f64::Length>,
        velocity: Vec2<unit::f64::Velocity>,
        acceleration: Vec2<unit::f64::Acceleration>,
        mass: unit::mass::kilogram,
        shape: Shape,
    ) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            mass,
            shape,
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        let physics_delta_time = unit::time::Time::new::<unit::time::second>(delta_time);
        self.velocity += self.acceleration * physics_delta_time;
        self.position += self.velocity * physics_delta_time;
    }
}
