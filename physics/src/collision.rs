pub mod default;

use crate::body::{AngularState, Body, LinearState, Shape};
use crate::id_map::IdMap;
use crate::id_pool::Id;
use crate::types::math::*;
use std::collections::HashMap;

pub trait CollisionPipeline {
    fn init(&mut self, bodies: &mut IdMap<Body>);

    fn handle(&mut self, bodies: &mut IdMap<Body>);
}

pub trait CollisionDetection {
    fn init(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>);

    fn detect(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>) -> Vec<CollisionData>;
}

pub trait BroadPhase {
    // Remove into another trait
    fn init(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>);

    // Better name
    fn cull(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>) -> Vec<[Id; 2]>;
}

pub trait NarrowPhase {
    fn init(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>);

    fn detect(&mut self, body_pairs: Vec<[Id; 2]>, bodies: &mut IdMap<Body>) -> Vec<CollisionData>;
}

pub trait CollisionResolution {
    fn init(&mut self, bodies: &mut IdMap<Body>);

    fn resolve(&mut self, collisions: Vec<CollisionData>, bodies: &mut IdMap<Body>);
}

#[derive(Debug)]
pub struct CollisionData {
    pub bodies: [Id; 2],
    // Maybe use Point instead
    pub point: Vector<f64>,
    pub normal: Vector<f64>,
    pub depth: f64,
}
