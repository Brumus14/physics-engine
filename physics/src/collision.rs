pub mod default;

use crate::body::{LinearState, Shape};
use crate::id_pool::Id;
use crate::types::math::*;
use std::collections::HashMap;

pub trait CollisionDetection {
    fn new() -> Self;

    fn detect(
        &mut self,
        objects: Vec<Id>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData>;
}

pub trait CollisionResolution {
    fn new() -> Self;

    fn resolve(
        &mut self,
        collisions: Vec<CollisionData>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    );
}

pub trait BroadPhase {
    fn new() -> Self;

    // Better name
    fn cull(&mut self, objects: Vec<Id>) -> Vec<[Id; 2]>;
}

pub trait NarrowPhase {
    fn new() -> Self;

    fn detect(
        &mut self,
        object_pairs: Vec<[Id; 2]>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData>;
}

#[derive(Debug)]
pub struct CollisionData {
    objects: [Id; 2],
    // Maybe use Point instead
    point: Vector<f64>,
    normal: Vector<f64>,
    depth: f64,
}
