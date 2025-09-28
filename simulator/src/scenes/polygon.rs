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

    for i in 0..30 {
        bodies.push(spawn_physics_body(
            commands,
            meshes,
            materials,
            physics_world,
            Body::new_rigid(
                LinearState::new(
                    Vector::new(
                        rng.random_range(-800.0..800.0),
                        rng.random_range(-400.0..1000.0),
                    ),
                    Vector::zeros(),
                    1.0,
                ),
                0.99,
                AngularState::new(rng.random_range(0.0..f64::consts::TAU), 0.0, 1000.0),
                Shape::new_polygon(vec![
                    Vector::new(40.0, 0.0),
                    Vector::new(30.0, 20.0),
                    Vector::new(10.0, 30.0),
                    Vector::new(-10.0, 35.0),
                    Vector::new(-30.0, 20.0),
                    Vector::new(-40.0, 5.0),
                    Vector::new(-30.0, -15.0),
                    Vector::new(-10.0, -25.0),
                    Vector::new(15.0, -20.0),
                    Vector::new(30.0, -10.0),
                ]),
            ),
            Color::linear_rgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            ),
        ));
    }

    for i in 0..30 {
        bodies.push(spawn_physics_body(
            commands,
            meshes,
            materials,
            physics_world,
            Body::new_rigid(
                LinearState::new(
                    Vector::new(
                        rng.random_range(-800.0..800.0),
                        rng.random_range(-400.0..1000.0),
                    ),
                    Vector::zeros(),
                    1.0,
                ),
                0.99,
                AngularState::new(rng.random_range(0.0..f64::consts::TAU), 0.0, 1000.0),
                Shape::new_polygon(vec![
                    Vector::new(50.0, -10.0),
                    Vector::new(35.0, 25.0),
                    Vector::new(10.0, 40.0),
                    Vector::new(-15.0, 30.0),
                    Vector::new(-40.0, 15.0),
                    Vector::new(-45.0, -10.0),
                    Vector::new(-20.0, -25.0),
                ]),
            ),
            Color::linear_rgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            ),
        ));
    }

    for i in 0..30 {
        bodies.push(spawn_physics_body(
            commands,
            meshes,
            materials,
            physics_world,
            Body::new_rigid(
                LinearState::new(
                    Vector::new(
                        rng.random_range(-800.0..800.0),
                        rng.random_range(-400.0..1000.0),
                    ),
                    Vector::zeros(),
                    1.0,
                ),
                0.99,
                AngularState::new(rng.random_range(0.0..f64::consts::TAU), 0.0, 1000.0),
                Shape::new_polygon(vec![
                    Vector::new(0.0, 15.0),
                    Vector::new(60.0, 0.0),
                    Vector::new(0.0, -15.0),
                ]),
            ),
            Color::linear_rgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            ),
        ));
    }

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
            [bodies.clone(), vec![ground]].concat(),
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
            [bodies.clone(), vec![ground]].concat(),
        )));
}
