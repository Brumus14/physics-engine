use crate::{effector::Spring, id_pool::Id, types::math::*};

#[derive(Clone)]
pub enum Body {
    // Particle {
    Point {
        linear: LinearState,
    },
    Rigid {
        linear: LinearState,
        restitution: f64,
        angular: AngularState,
        shape: Shape,
    },
}

#[derive(Clone)]
pub struct LinearState {
    pub position: Vector<f64>,
    pub velocity: Vector<f64>,
    pub force: Vector<f64>,
    pub mass: f64,
}

impl LinearState {
    pub fn new(position: Vector<f64>, velocity: Vector<f64>, mass: f64) -> Self {
        Self {
            position,
            velocity,
            force: Vector::zeros(),
            mass,
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Shape {
    Circle(f64),
    Rectangle(Vector<f64>),
    Polygon(Vec<Vector<f64>>),
}

// #[derive(Clone)]
// pub enum Shape {
//     Circle(f64),
//     Polygon {
//         points: Vec<Vector<f64>>,
//         normals: Vec<Vector<f64>>,
//     },
// }
//
// impl Shape {
//     pub fn new_circle(radius: f64) -> Self {
//         Shape::Circle(radius)
//     }
//
//     // pub fn new_rectangle(size: Vector<f64>) -> Self {
//     //     Shape::Polygon {
//     //         points: vec![],
//     //         normals: (),
//     //     }
//     // }
// }
