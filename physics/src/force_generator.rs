use std::collections::HashMap;

use crate::{
    body::{Body, Linear},
    id_pool::Id,
    types::math::*,
};

pub trait ForceGenerator: Send + Sync {
    fn apply(&self, bodies: &mut HashMap<Id, Box<dyn Body>>);
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
    fn apply(&self, bodies: &mut HashMap<Id, Box<dyn Body>>) {
        for id in self.bodies.iter() {
            if let Some(body) = bodies.get_mut(id) {
                body.linear_mut().force += self.force;
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
    fn apply(&self, bodies: &mut HashMap<Id, T>) {
        for id in self.bodies.iter() {
            if let Some(body) = bodies.get_mut(id) {
                let linear = body.linear_mut();
                linear.force += self.acceleration * linear.mass;
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
                let (a_id, b_id) = (self.bodies[i], self.bodies[j]);
                let (a, b) = (
                    bodies.get(&a_id).unwrap().linear(),
                    bodies.get(&b_id).unwrap().linear(),
                );

                let direction = b.position - a.position;
                // TODO: Review this
                let distance_squared = direction.norm_squared().max(0.0001);

                let force = direction.normalize() * (self.gravitational_constant * a.mass * b.mass)
                    / distance_squared;

                bodies.get_mut(&a_id).unwrap().linear_mut().force += force;
                bodies.get_mut(&b_id).unwrap().linear_mut().force -= force;
            }
        }
    }
}
