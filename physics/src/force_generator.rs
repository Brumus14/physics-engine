use std::collections::HashMap;

use nalgebra::distance_squared;

use crate::{object::Object, types::math::*, world::ObjectId};

pub trait ForceGenerator: Send + Sync {
    fn apply(&self, objects: &mut HashMap<ObjectId, Object>);
}

pub struct ConstantForce {
    objects: Vec<ObjectId>,
    force: Vector<f64>,
}

impl ConstantForce {
    pub fn new(objects: Vec<ObjectId>, force: Vector<f64>) -> Self {
        Self { objects, force }
    }
}

impl ForceGenerator for ConstantForce {
    fn apply(&self, objects: &mut HashMap<ObjectId, Object>) {
        for id in self.objects.iter() {
            if let Some(object) = objects.get_mut(id) {
                object.force += self.force;
            }
        }
    }
}

pub struct ConstantAcceleration {
    objects: Vec<ObjectId>,
    acceleration: Vector<f64>,
}

impl ConstantAcceleration {
    pub fn new(objects: Vec<ObjectId>, acceleration: Vector<f64>) -> Self {
        Self {
            objects,
            acceleration,
        }
    }
}

impl ForceGenerator for ConstantAcceleration {
    fn apply(&self, objects: &mut HashMap<ObjectId, Object>) {
        for id in self.objects.iter() {
            if let Some(object) = objects.get_mut(id) {
                object.force += self.acceleration * object.mass;
            }
        }
    }
}

pub struct Gravity {
    objects: Vec<ObjectId>,
    gravitational_constant: f64,
}

impl Gravity {
    pub fn new(objects: Vec<ObjectId>, gravitational_constant: f64) -> Self {
        Self {
            objects,
            gravitational_constant,
        }
    }
}

impl ForceGenerator for Gravity {
    fn apply(&self, objects: &mut HashMap<ObjectId, Object>) {
        for i in 0..self.objects.len() {
            for j in (i + 1)..self.objects.len() {
                let (a, b) = (
                    objects.get(&self.objects[i]).unwrap(),
                    objects.get(&self.objects[j]).unwrap(),
                );

                let direction = b.position - a.position;
                // TODO: Review this
                let distance_squared = direction.norm_squared().max(0.0001);

                let force = direction.normalize() * (self.gravitational_constant * a.mass * b.mass)
                    / distance_squared;

                objects.get_mut(&self.objects[i]).unwrap().force += force;
                objects.get_mut(&self.objects[j]).unwrap().force -= force;
            }
        }
    }
}
