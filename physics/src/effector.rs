use std::{any::Any, collections::HashMap};

use crate::{
    body::{AngularState, Body, LinearState},
    id_map::{Id, IdMap},
    types::math::*,
};

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Effector> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Maybe add init
pub trait Effector: Any + AsAny + Send + Sync {
    fn apply(&self, bodies: &mut IdMap<Body>);
}

pub struct ConstantForce {
    pub bodies: Vec<Id>,
    pub force: Vector<f64>,
}

impl ConstantForce {
    pub fn new(bodies: Vec<Id>, force: Vector<f64>) -> Self {
        Self { bodies, force }
    }
}

impl Effector for ConstantForce {
    fn apply(&self, bodies: &mut IdMap<Body>) {
        for id in &self.bodies {
            if let Some(body) = bodies.get_mut(*id) {
                body.linear.force += self.force;
            }
        }
    }
}

pub struct ConstantAcceleration {
    pub bodies: Vec<Id>,
    pub acceleration: Vector<f64>,
}

impl ConstantAcceleration {
    pub fn new(bodies: Vec<Id>, acceleration: Vector<f64>) -> Self {
        Self {
            bodies,
            acceleration,
        }
    }
}

impl Effector for ConstantAcceleration {
    fn apply(&self, bodies: &mut IdMap<Body>) {
        for id in &self.bodies {
            if let Some(body) = bodies.get_mut(*id) {
                body.linear.force += self.acceleration * body.linear.mass;
            }
        }
    }
}

pub struct Gravity {
    pub bodies: Vec<Id>,
    pub gravitational_constant: f64,
}

impl Gravity {
    pub fn new(bodies: Vec<Id>, gravitational_constant: f64) -> Self {
        Self {
            bodies,
            gravitational_constant,
        }
    }
}

impl Effector for Gravity {
    fn apply(&self, bodies: &mut IdMap<Body>) {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let (a_id, b_id) = (self.bodies[i], self.bodies[j]);
                let Some(a) = bodies.get(a_id) else { continue };
                let Some(b) = bodies.get(b_id) else { continue };

                let direction = b.linear.position - a.linear.position;
                // TODO: Review this
                let distance_squared = direction.norm_squared().max(0.0001);

                let force = direction.normalize()
                    * (self.gravitational_constant * a.linear.mass * b.linear.mass)
                    / distance_squared;

                bodies.get_mut(a_id).unwrap().linear.force += force;
                bodies.get_mut(b_id).unwrap().linear.force -= force;
            }
        }
    }
}

pub struct ConstantTorque {
    pub bodies: Vec<Id>,
    pub torque: f64,
}

impl ConstantTorque {
    pub fn new(bodies: Vec<Id>, torque: f64) -> Self {
        Self { bodies, torque }
    }
}

impl Effector for ConstantTorque {
    fn apply(&self, bodies: &mut IdMap<Body>) {
        for id in &self.bodies {
            if let Some(body) = bodies.get_mut(*id) {
                body.angular.torque += self.torque;
            }
        }
    }
}

#[derive(Clone)]
pub struct Spring {
    pub bodies: [Id; 2],
    pub length: f64,
    pub elasticity: f64,
}

impl Spring {
    pub fn new(bodies: [Id; 2], length: f64, elasticity: f64) -> Self {
        Self {
            bodies,
            length,
            elasticity,
        }
    }

    pub fn new_auto_length(body_ids: [Id; 2], elasticity: f64, bodies: &mut IdMap<Body>) -> Self {
        let (a, b) = (
            bodies.get(body_ids[0]).unwrap(),
            bodies.get(body_ids[1]).unwrap(),
        );
        let length = a.linear.position.metric_distance(&b.linear.position);

        Self {
            bodies: body_ids,
            length,
            elasticity,
        }
    }
}

impl Effector for Spring {
    fn apply(&self, bodies: &mut IdMap<Body>) {
        let (a_id, b_id) = (self.bodies[0], self.bodies[1]);
        let Some(a) = bodies.get(a_id) else {
            return;
        };

        let Some(b) = bodies.get(b_id) else {
            return;
        };

        let length = a.linear.position.metric_distance(&b.linear.position);
        let force = self.elasticity * (length - self.length);
        let direction = (b.linear.position - a.linear.position).normalize();

        // Add get many mut
        bodies.get_mut(a_id).unwrap().linear.force += force * direction;
        bodies.get_mut(b_id).unwrap().linear.force -= force * direction;
    }
}

pub struct Drag {
    pub bodies: Vec<Id>,
    // Better name?
    pub coefficient: f64,
}

impl Drag {
    pub fn new(bodies: Vec<Id>, coefficient: f64) -> Self {
        Self {
            bodies,
            coefficient,
        }
    }
}

// Doesn't account for area
impl Effector for Drag {
    fn apply(&self, bodies: &mut IdMap<Body>) {
        for id in &self.bodies {
            if let Some(body) = bodies.get_mut(*id) {
                body.linear.force += -(1.0 / 2.0)
                    * body.linear.velocity.norm()
                    * body.linear.velocity
                    * self.coefficient;
            }
        }
    }
}
