use std::any::Any;

use crate::{
    body::Body,
    id_map::{Id, IdMap},
    types::{math::*, *},
};

// Maybe add init
pub trait Integrator {
    fn step(&mut self, delta_time: f64, bodies: &mut IdMap<Body>);
}

pub struct ExplicitEuler {
    pub bodies: Vec<Id>,
}

impl ExplicitEuler {
    pub fn new(bodies: Vec<Id>) -> Self {
        Self { bodies }
    }

    fn add_body(&mut self, id: Id) {
        if !self.bodies.contains(&id) {
            self.bodies.push(id);
        }
    }

    fn remove_body(&mut self, id: Id) {
        if let Some(index) = self.bodies.iter().position(|i| *i == id) {
            self.bodies.remove(index);
        }
    }
}

impl Integrator for ExplicitEuler {
    fn step(&mut self, delta_time: f64, bodies: &mut IdMap<Body>) {
        for id in &self.bodies {
            if let Some(body) = bodies.get_mut(*id) {
                let linear = &mut body.linear;

                linear.position += linear.velocity * delta_time;
                linear.velocity += (linear.force / linear.mass) * delta_time;
                // Should this be in the integrator?
                linear.force = Vector::zeros();

                let angular = &mut body.angular;

                // Is this dodgy
                if angular.inertia == 0.0 {
                    continue;
                };

                angular.orientation += angular.velocity * delta_time;
                angular.velocity += (angular.torque / angular.inertia) * delta_time;
                angular.torque = 0.0;
            }
        }
    }
}

pub struct SemiImplicitEuler {
    pub bodies: Vec<Id>,
}

impl SemiImplicitEuler {
    pub fn new(bodies: Vec<Id>) -> Self {
        Self { bodies }
    }

    fn add_body(&mut self, id: Id) {
        if !self.bodies.contains(&id) {
            self.bodies.push(id);
        }
    }

    fn remove_body(&mut self, id: Id) {
        if let Some(index) = self.bodies.iter().position(|i| *i == id) {
            self.bodies.remove(index);
        }
    }
}

impl Integrator for SemiImplicitEuler {
    fn step(&mut self, delta_time: f64, bodies: &mut IdMap<Body>) {
        for id in &self.bodies {
            if let Some(body) = bodies.get_mut(*id) {
                let linear = &mut body.linear;

                linear.velocity += (linear.force / linear.mass) * delta_time;
                linear.position += linear.velocity * delta_time;

                let angular = &mut body.angular;

                if angular.inertia == 0.0 {
                    continue;
                };

                angular.velocity += (angular.torque / angular.inertia) * delta_time;
                angular.orientation += angular.velocity * delta_time;
            }
        }
    }
}
