use std::f64;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use i_triangle::float::triangulatable::Triangulatable;
use physics::{
    Id,
    body::{AngularState, Body, LinearState, Shape},
    collision::{
        NarrowPhase,
        default::{
            DefaultCollisionDetector, DefaultCollisionPipeline, DefaultCollisionResolver,
            DefaultNarrowPhase,
        },
    },
    effector::{ConstantAcceleration, ConstantTorque, Gravity},
    types::math::*,
    world::World,
};
use rand::Rng;

const PARTICLE_SIZE: f32 = 10.0;

#[derive(Resource)]
struct PhysicsWorld {
    world: World,
}

#[derive(Component)]
struct PhysicsObject {
    // Switch from usize to some handle
    id: Id,
}

fn shape_to_mesh(shape: &Shape) -> Mesh {
    match shape {
        Shape::Circle(radius) => Circle::new(radius.clone() as f32).into(),
        Shape::Rectangle(size) => {
            Rectangle::new(size.x.clone() as f32, size.y.clone() as f32).into()
        }
        Shape::Polygon(points) => {
            let shape: Vec<[f64; 2]> = points.iter().map(|p| [p[0], p[1]]).collect();
            let triangulation = shape.triangulate().to_triangulation();

            Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                triangulation
                    .points
                    .iter()
                    .map(|p| [p[0] as f32, p[1] as f32, 0.0])
                    .collect::<Vec<[f32; 3]>>(),
            )
            .with_inserted_indices(Indices::U32(triangulation.indices))
        }
    }
}

fn spawn_physics_object(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    physics_world: &mut ResMut<PhysicsWorld>,
    body: Body,
    colour: Color,
) -> Id {
    let mesh: Mesh;
    let transform: Transform;

    match &body {
        Body::Particle { linear } => {
            mesh = Circle::new(PARTICLE_SIZE).into();
            transform =
                Transform::from_xyz(linear.position.x as f32, linear.position.y as f32, 0.0);
        }
        Body::Rigid {
            linear,
            angular,
            shape,
        } => {
            mesh = shape_to_mesh(shape);
            transform =
                Transform::from_xyz(linear.position.x as f32, linear.position.y as f32, 0.0)
                    .with_rotation(Quat::from_rotation_z(-angular.rotation as f32));
        }
    }

    let physics_id = physics_world.world.add_body(body);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(colour)),
        transform,
        PhysicsObject { id: physics_id },
    ));

    physics_id
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (startup_physics, startup).chain())
        .add_systems(Update, (update_physics, update).chain())
        .run();
}

fn startup_physics(mut commands: Commands) {
    let physics_world = World::new();

    commands.insert_resource(PhysicsWorld {
        world: physics_world,
    });
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut physics_world: ResMut<PhysicsWorld>,
) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let a = spawn_physics_object(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Body::Rigid {
            linear: LinearState::new(Vector::new(-100.0, 0.0), Vector::new(0.0, 0.0), 1.0, 1.0),
            angular: AngularState::new(0.0, 0.0, 1.0),
            shape: Shape::Rectangle(Vector::new(200.0, 100.0)),
        },
        Color::linear_rgb(0.0, 1.0, 0.0),
    );

    let b = spawn_physics_object(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Body::Rigid {
            linear: LinearState::new(Vector::new(45.0, 0.0), Vector::new(0.0, 0.0), 1.0, 1.0),
            angular: AngularState::new(0.0, 0.0, 1.0),
            shape: Shape::Rectangle(Vector::new(100.0, 150.0)),
        },
        Color::linear_rgb(1.0, 0.0, 0.0),
    );

    // physics_world
    //     .world
    //     .add_effector(Box::new(Gravity::new(vec![a, b, c], 400000.0)));

    physics_world
        .world
        .add_collision_pipeline(Box::new(DefaultCollisionPipeline::new(vec![a])));
}

fn update_physics(
    mut physics_world: ResMut<PhysicsWorld>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &PhysicsObject)>,
) {
    let physics_world = &mut physics_world.world;

    physics_world.apply_effectors();
    physics_world.step(time.delta_secs_f64());
    physics_world.handle_collisions();

    query.iter_mut().for_each(|(mut transform, physics)| {
        let position = physics_world.get_linear(physics.id).unwrap().position;
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;

        if let Some(angular) = physics_world.get_angular(physics.id) {
            transform.rotation = Quat::from_rotation_z(-angular.rotation as f32);
        };
    });
}

fn update() {}
