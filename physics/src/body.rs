use crate::types::math::*;

pub enum Body {
    Particle(Particle),
    Rigid(Rigid),
}

pub struct Particle {
    position: Vector<f64>,
    velocity: Vector<f64>,
    force: Vector<f64>,
    mass: f64,
}

pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Polygon { points: Vec<[f64; 2]> },
}

pub struct Rigid {
    position: Vector<f64>,
    velocity: Vector<f64>,
    force: Vector<f64>,
    mass: f64,
    rotation: f64,
    angular_velocity: f64,
    torque: f64,
    inertia: f64,
    shape: Shape,
}
