use std::f64;

use bevy::prelude::*;
use physics::{
    body::{AngularState, Body, LinearState, Shape},
    collision::default::DefaultCollisionPipeline,
    effector::ConstantAcceleration,
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

    let mut bodies: Vec<Id> = Vec::new();

    for i in 0..10 {
        bodies.push(spawn_physics_body(
            commands,
            meshes,
            materials,
            physics_world,
            Body::new_rigid(
                LinearState::new(
                    Vector::new(0.0, 60.0 * (i as f64).powf(1.05)),
                    Vector::zeros(),
                    1.0,
                ),
                0.99,
                AngularState::new(0.0, 0.0, 1000.0),
                Shape::new_rectangle(Vector::new(100.0, 50.0)),
            ),
            Color::linear_rgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            ),
        ));
    }

    let circle = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_rigid(
            LinearState::new(Vector::new(-800.0, -200.0), Vector::new(200.0, 0.0), 100.0),
            1.0,
            AngularState::new(0.0, 0.0, 1000.0),
            Shape::new_circle(50.0),
        ),
        Color::WHITE,
    );

    let ground = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_rigid(
            LinearState::new(Vector::new(0.0, -500.0), Vector::zeros(), f64::INFINITY),
            0.3,
            AngularState::new(0.0, 0.0, f64::INFINITY),
            Shape::new_rectangle(Vector::new(1600.0, 50.0)),
        ),
        Color::WHITE,
    );

    physics_world
        .world
        .add_integrator(Box::new(SemiImplicitEuler::new(
            [bodies.clone(), vec![circle, ground]].concat(),
        )));

    physics_world
        .world
        .add_effector(Box::new(ConstantAcceleration::new(
            bodies.clone(),
            Vector::new(0.0, -200.0),
        )));

    physics_world
        .world
        .add_collision_pipeline(Box::new(DefaultCollisionPipeline::new(
            [bodies.clone(), vec![circle, ground]].concat(),
        )));
}
