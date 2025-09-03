use std::{any::Any, collections::HashMap};

use crate::{
    body::{AngularState, LinearState},
    id_pool::Id,
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

pub trait Effector: Any + AsAny + Send + Sync {
    fn apply(
        &self,
        linear_states: &mut HashMap<Id, LinearState>,
        angular_states: &mut HashMap<Id, AngularState>,
    );
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
    fn apply(
        &self,
        linear_states: &mut HashMap<Id, LinearState>,
        _: &mut HashMap<Id, AngularState>,
    ) {
        for id in self.bodies.iter() {
            linear_states.get_mut(id).unwrap().force += self.force;
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
    fn apply(
        &self,
        linear_states: &mut HashMap<Id, LinearState>,
        _: &mut HashMap<Id, AngularState>,
    ) {
        for id in self.bodies.iter() {
            let linear = linear_states.get_mut(id).unwrap();
            linear.force += self.acceleration * linear.mass;
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
    fn apply(
        &self,
        linear_states: &mut HashMap<Id, LinearState>,
        _: &mut HashMap<Id, AngularState>,
    ) {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let (a_id, b_id) = (self.bodies[i], self.bodies[j]);
                let (a, b) = (
                    linear_states.get(&a_id).unwrap(),
                    linear_states.get(&b_id).unwrap(),
                );

                let direction = b.position - a.position;
                // TODO: Review this
                let distance_squared = direction.norm_squared().max(0.0001);

                let force = direction.normalize() * (self.gravitational_constant * a.mass * b.mass)
                    / distance_squared;

                linear_states.get_mut(&a_id).unwrap().force += force;
                linear_states.get_mut(&b_id).unwrap().force -= force;
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
    fn apply(
        &self,
        _: &mut HashMap<Id, LinearState>,
        angular_states: &mut HashMap<Id, AngularState>,
    ) {
        for id in self.bodies.iter() {
            let angular = angular_states.get_mut(id).unwrap();
            angular.torque += self.torque;
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
}

impl Effector for Spring {
    fn apply(
        &self,
        linear_states: &mut HashMap<Id, LinearState>,
        _angular_states: &mut HashMap<Id, AngularState>,
    ) {
        let [a_linear, b_linear] = linear_states
            .get_disjoint_mut([&self.bodies[0], &self.bodies[1]])
            .map(|l| l.unwrap());
        let length = a_linear.position.metric_distance(&b_linear.position);
        let force = self.elasticity * (length - self.length);
        let direction = (b_linear.position - a_linear.position).normalize();
        a_linear.force += force * direction;
        b_linear.force -= force * direction;
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
    fn apply(
        &self,
        linear_states: &mut HashMap<Id, LinearState>,
        _angular_states: &mut HashMap<Id, AngularState>,
    ) {
        for id in self.bodies.iter() {
            let linear = linear_states.get_mut(id).unwrap();
            linear.force +=
                -(1.0 / 2.0) * linear.velocity.norm() * linear.velocity * self.coefficient;
        }
    }
}
