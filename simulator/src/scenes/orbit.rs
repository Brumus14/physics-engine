use std::f64;

use bevy::prelude::*;
use physics::{
    body::{AngularState, Body, LinearState, Shape},
    collision::default::DefaultCollisionPipeline,
    effector::{ConstantAcceleration, Gravity},
    id_map::Id,
    integrator::SemiImplicitEuler,
    types::math::Vector,
    world::World,
};
use rand::Rng;

use crate::physics_helpers::{PhysicsWorld, spawn_physics_body};

pub fn load(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    physics_world: &mut ResMut<PhysicsWorld>,
) {
    let mut rng = rand::rng();

    let sun = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_rigid(
            LinearState::new(Vector::new(0.0, 0.0), Vector::zeros(), 10000.0),
            1.0,
            AngularState::new(0.0, 0.0, 1000.0),
            Shape::new_circle(30.0),
        ),
        Color::linear_rgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        ),
    );

    let earth = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_rigid(
            LinearState::new(Vector::new(100.0, 0.0), Vector::new(0.0, 10.0), 10.0),
            1.0,
            AngularState::new(rng.random_range(0.0..f64::consts::TAU), 0.0, 1000.0),
            Shape::new_circle(5.0),
        ),
        Color::linear_rgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        ),
    );

    physics_world
        .world
        .add_integrator(Box::new(SemiImplicitEuler::new(vec![sun, earth])));

    physics_world
        .world
        .add_effector(Box::new(Gravity::new(vec![sun, earth], 1.0)));
}
