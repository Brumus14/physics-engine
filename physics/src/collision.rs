use crate::id_pool::Id;
use crate::types::math::*;

pub struct CollisionData {
    // Maybe use Point instead
    point: Vector<f64>,
    normal: Vector<f64>,
    depth: f64,
}

pub trait BroadPhase {
    fn cull(objects: Vec<Id>) -> Vec<[Id; 2]>;
}

pub trait NarrowPhase {
    fn detect(object_pairs: Vec<[Id; 2]>) -> Vec<CollisionData>;
}

pub struct DefaultBroadPhase {}

impl BroadPhase for DefaultBroadPhase {
    fn cull(objects: Vec<Id>) -> Vec<[Id; 2]> {
        let mut pairs = Vec::new();

        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                pairs.push([objects[i], objects[j]]);
            }
        }

        pairs
    }
}

pub struct CircleNarrowPhase {}
