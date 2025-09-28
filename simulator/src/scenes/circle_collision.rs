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

    for i in 0..1200 {
        bodies.push(spawn_physics_body(
            commands,
            meshes,
            materials,
            physics_world,
            Body::new_rigid(
                LinearState::new(
                    Vector::new(
                        rng.random_range(-600.0..600.0),
                        rng.random_range(-600.0..600.0),
                    ),
                    Vector::zeros(),
                    1.0,
                ),
                0.99,
                AngularState::new(rng.random_range(0.0..f64::consts::TAU), 0.0, 1000.0),
                Shape::new_circle(rng.random_range(10.0..15.0)),
            ),
            Color::linear_rgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            ),
        ));
    }

    let circle1 = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_rigid(
            LinearState::new(
                Vector::new(-1000.0, 0.0),
                Vector::new(400.0, 0.0),
                f64::INFINITY,
            ),
            1.0,
            AngularState::new(0.0, 0.0, 1000.0),
            Shape::new_circle(50.0),
        ),
        Color::WHITE,
    );

    let circle2 = spawn_physics_body(
        commands,
        meshes,
        materials,
        physics_world,
        Body::new_rigid(
            LinearState::new(
                Vector::new(0.0, -1400.0),
                Vector::new(0.0, 400.0),
                f64::INFINITY,
            ),
            1.0,
            AngularState::new(0.0, 0.0, 1000.0),
            Shape::new_circle(50.0),
        ),
        Color::WHITE,
    );

    physics_world
        .world
        .add_integrator(Box::new(SemiImplicitEuler::new(
            [bodies.clone(), vec![circle1, circle2]].concat(),
        )));

    physics_world
        .world
        .add_collision_pipeline(Box::new(DefaultCollisionPipeline::new(
            [bodies.clone(), vec![circle1, circle2]].concat(),
        )));
}
