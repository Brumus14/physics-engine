pub mod default;

use crate::body::{AngularState, LinearState, Shape};
use crate::id_pool::Id;
use crate::types::math::*;
use std::collections::HashMap;

pub trait CollisionPipeline {
    fn handle(
        &mut self,
        linear_states: &mut HashMap<Id, LinearState>,
        restitutions: &HashMap<Id, f64>,
        angular_states: &mut HashMap<Id, AngularState>,
        shapes: &HashMap<Id, Shape>,
    );
}

pub trait CollisionDetection {
    fn detect(
        &mut self,
        bodies: &Vec<Id>,
        linear_states: &HashMap<Id, LinearState>,
        angular_states: &HashMap<Id, AngularState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData>;
}

pub trait BroadPhase {
    // Better name
    fn cull(&mut self, bodies: &Vec<Id>) -> Vec<[Id; 2]>;
}

pub trait NarrowPhase {
    fn detect(
        &mut self,
        bodies: &Vec<Id>,
        body_pairs: Vec<[Id; 2]>,
        linear_states: &HashMap<Id, LinearState>,
        angular_states: &HashMap<Id, AngularState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData>;
}

pub trait CollisionResolution {
    fn resolve(
        &mut self,
        collisions: Vec<CollisionData>,
        linear_states: &mut HashMap<Id, LinearState>,
        restitutions: &HashMap<Id, f64>,
        shapes: &HashMap<Id, Shape>,
    );
}

#[derive(Debug)]
pub struct CollisionData {
    pub bodies: [Id; 2],
    // Maybe use Point instead
    pub point: Vector<f64>,
    pub normal: Vector<f64>,
    pub depth: f64,
}
