use std::f64;

use bevy::prelude::*;
use physics::{
    body::{AngularState, Body, LinearState, Shape},
    collision::default::DefaultCollisionPipeline,
    effector::{ConstantAcceleration, Spring},
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

    let fixed = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_particle(
            LinearState::new(Vector::new(0.0, 200.0), Vector::zeros(), f64::INFINITY),
            1.0,
        ),
        Color::linear_rgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        ),
    );

    let point1 = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_particle(
            LinearState::new(Vector::new(100.0, 0.0), Vector::zeros(), 1.0),
            1.0,
        ),
        Color::linear_rgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        ),
    );

    let point2 = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_particle(
            LinearState::new(Vector::new(0.0, -150.0), Vector::zeros(), 1.0),
            1.0,
        ),
        Color::linear_rgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        ),
    );

    physics_world
        .world
        .add_integrator(Box::new(SemiImplicitEuler::new(vec![
            fixed, point1, point2,
        ])));

    physics_world
        .world
        .add_effector(Box::new(Spring::new([fixed, point1], 150.0, 20.0)));

    physics_world
        .world
        .add_effector(Box::new(Spring::new([point1, point2], 150.0, 20.0)));

    physics_world
        .world
        .add_effector(Box::new(ConstantAcceleration::new(
            vec![point1, point2],
            Vector::new(0.0, -200.0),
        )));
}
