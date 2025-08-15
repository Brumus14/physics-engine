use std::collections::HashMap;

use crate::{body::Body, id_pool::Id, types::math::*};

pub trait ForceGenerator: Send + Sync {
    fn apply(&self, bodies: &mut HashMap<Id, Body>);
}

pub struct ConstantForce {
    bodies: Vec<Id>,
    force: Vector<f64>,
}

impl ConstantForce {
    pub fn new(bodies: Vec<Id>, force: Vector<f64>) -> Self {
        Self { bodies, force }
    }
}

impl ForceGenerator for ConstantForce {
    fn apply(&self, bodies: &mut HashMap<Id, Body>) {
        for id in self.bodies.iter() {
            if let Some(body) = bodies.get_mut(id) {
                body.force += self.force;
            }
        }
    }
}

pub struct ConstantAcceleration {
    bodies: Vec<Id>,
    acceleration: Vector<f64>,
}

impl ConstantAcceleration {
    pub fn new(bodies: Vec<Id>, acceleration: Vector<f64>) -> Self {
        Self {
            bodies,
            acceleration,
        }
    }
}

impl ForceGenerator for ConstantAcceleration {
    fn apply(&self, bodies: &mut HashMap<Id, Body>) {
        for id in self.bodies.iter() {
            if let Some(body) = bodies.get_mut(id) {
                body.force += self.acceleration * body.mass;
            }
        }
    }
}

pub struct Gravity {
    bodies: Vec<Id>,
    gravitational_constant: f64,
}

impl Gravity {
    pub fn new(bodies: Vec<Id>, gravitational_constant: f64) -> Self {
        Self {
            bodies,
            gravitational_constant,
        }
    }
}

impl ForceGenerator for Gravity {
    fn apply(&self, bodies: &mut HashMap<Id, Body>) {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let (a, b) = (
                    bodies.get(&self.bodies[i]).unwrap(),
                    bodies.get(&self.bodies[j]).unwrap(),
                );

                let direction = b.position - a.position;
                // TODO: Review this
                let distance_squared = direction.norm_squared().max(0.0001);

                let force = direction.normalize() * (self.gravitational_constant * a.mass * b.mass)
                    / distance_squared;

                bodies.get_mut(&self.bodies[i]).unwrap().force += force;
                bodies.get_mut(&self.bodies[j]).unwrap().force -= force;
            }
        }
    }
}
