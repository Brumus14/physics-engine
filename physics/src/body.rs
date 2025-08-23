use crate::{effector::Spring, types::math::*};

pub enum Body {
    Particle {
        linear: LinearState,
    },
    Rigid {
        linear: LinearState,
        angular: AngularState,
        shape: Shape,
    },
    // Mass-spring
    Soft {
        points: Vec<LinearState>,
        springs: Vec<Spring>,
    },
}

pub struct LinearState {
    pub position: Vector<f64>,
    pub velocity: Vector<f64>,
    pub force: Vector<f64>,
    pub mass: f64,
    // Move to rigid?
    pub restitution: f64,
}

impl LinearState {
    pub fn new(position: Vector<f64>, velocity: Vector<f64>, mass: f64, restitution: f64) -> Self {
        Self {
            position,
            velocity,
            force: Vector::zeros(),
            mass,
            // Should this be moved?
            restitution,
        }
    }
}

pub struct AngularState {
    pub rotation: f64,
    pub velocity: f64,
    pub torque: f64,
    pub inertia: f64,
}

impl AngularState {
    pub fn new(rotation: f64, velocity: f64, inertia: f64) -> Self {
        Self {
            rotation,
            velocity,
            torque: 0.0,
            inertia,
        }
    }
}

pub enum Shape {
    Circle(f64),
    Rectangle(Vector<f64>),
    Polygon(Vec<Vector<f64>>),
}
