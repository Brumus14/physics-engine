use crate::types::math::*;

pub trait Body {}

pub struct LinearState {
    pub position: Vector<f64>,
    pub velocity: Vector<f64>,
    pub force: Vector<f64>,
    pub mass: f64,
}

pub trait Linear {
    fn linear(&self) -> &LinearState;
    fn linear_mut(&mut self) -> &mut LinearState;
}

pub struct AngularState {
    pub rotation: f64,
    pub angular_velocity: f64,
    pub torque: f64,
    pub inertia: f64,
}

pub trait Angular {
    fn angular(&self) -> &AngularState;
    fn angular_mut(&mut self) -> &mut AngularState;
}

pub struct Particle {
    linear: LinearState,
}

impl Body for Particle {}

impl Linear for Particle {
    fn linear(&self) -> &LinearState {
        &self.linear
    }

    fn linear_mut(&mut self) -> &mut LinearState {
        &mut self.linear
    }
}

pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Polygon { points: Vec<[f64; 2]> },
}

pub struct RigidBody {
    linear: LinearState,
    angular: AngularState,
    shape: Shape,
}

impl Body for RigidBody {}

impl Linear for RigidBody {
    fn linear(&self) -> &LinearState {
        &self.linear
    }

    fn linear_mut(&mut self) -> &mut LinearState {
        &mut self.linear
    }
}

impl Angular for RigidBody {
    fn angular(&self) -> &AngularState {
        &self.angular
    }

    fn angular_mut(&mut self) -> &mut AngularState {
        &mut self.angular
    }
}
