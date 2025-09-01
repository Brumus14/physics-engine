use std::f64;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use i_triangle::float::triangulatable::Triangulatable;
use physics::{
    body::{AngularState, Body, BodyId, LinearState, Shape},
    collision::{
        NarrowPhase,
        default::{
            DefaultCollisionDetector, DefaultCollisionPipeline, DefaultCollisionResolver,
            DefaultNarrowPhase,
        },
    },
    effector::{ConstantAcceleration, Drag, Spring},
    id_pool::Id,
    types::math::*,
    world::World,
};
use rand::Rng;

const POINT_SIZE: f32 = 10.0;
const SPRING_SIZE: f32 = 10.0;

#[derive(Resource)]
struct PhysicsWorld {
    world: World,
}

#[derive(Component)]
struct ParticleBody(BodyId);

#[derive(Component)]
struct RigidBody(BodyId);

#[derive(Component)]
struct SoftBody(BodyId);

#[derive(Component)]
struct SoftBodyPoint(BodyId);

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
) -> BodyId {
    let physics_id = physics_world.world.add_body(body.clone());

    match &body {
        Body::Particle { linear } => {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(POINT_SIZE))),
                MeshMaterial2d(materials.add(colour)),
                Transform::from_xyz(linear.position.x as f32, linear.position.y as f32, 0.0),
                ParticleBody(physics_id.clone()),
            ));
        }
        Body::Rigid {
            linear,
            angular,
            shape,
        } => {
            commands.spawn((
                Mesh2d(meshes.add(shape_to_mesh(shape))),
                MeshMaterial2d(materials.add(colour)),
                Transform::from_xyz(linear.position.x as f32, linear.position.y as f32, 0.0)
                    .with_rotation(Quat::from_rotation_z(-angular.rotation as f32)),
                RigidBody(physics_id.clone()),
            ));
        }
        Body::Soft { points, springs } => {
            commands
                .spawn((SoftBody(physics_id.clone()),))
                .with_children(|body| {
                    for point in points {
                        body.spawn((
                            Mesh2d(meshes.add(Circle::new(POINT_SIZE))),
                            MeshMaterial2d(materials.add(colour)),
                            Transform::from_xyz(
                                point.position.x as f32,
                                point.position.y as f32,
                                0.0,
                            ),
                            SoftBodyPoint(physics_id.clone()),
                        ));
                    }

                    // for spring in springs {
                    //     body.spawn((
                    //         Mesh2d(meshes.add(Rectangle::new(SPRING_SIZE, 100.0))),
                    //         MeshMaterial2d(materials.add(colour)),
                    //         Transform::from_xyz(0.0, 0.0, 0.0)
                    //             .with_rotation(Quat::from_rotation_z(0.0)),
                    //     ));
                    // }
                });
        }
    }

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

    let rigid = spawn_physics_object(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Body::Rigid {
            linear: LinearState::new(Vector::new(-150.0, -100.0), Vector::new(1.0, 0.0), 1.0, 1.0),
            angular: AngularState::new(0.0, 0.0, 1.0),
            shape: Shape::Rectangle(Vector::new(100.0, 50.0)),
        },
        Color::WHITE,
    );

    let soft = spawn_physics_object(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Body::Soft {
            points: vec![
                LinearState::new(Vector::new(0.0, 0.0), Vector::new(0.0, 0.0), 1.0, 1.0),
                LinearState::new(Vector::new(100.0, 0.0), Vector::new(0.0, 0.0), 1.0, 1.0),
            ],
            springs: vec![Spring::new([0, 1], 100.0, 1.0)],
        },
        Color::WHITE,
    );
}

fn update_physics(
    mut physics_world: ResMut<PhysicsWorld>,
    time: Res<Time>,
    mut particle_query: Query<
        (&ParticleBody, &mut Transform),
        (Without<RigidBody>, Without<SoftBodyPoint>),
    >,
    mut rigid_body_query: Query<
        (&RigidBody, &mut Transform),
        (Without<ParticleBody>, Without<SoftBodyPoint>),
    >,
    mut soft_body_point_query: Query<
        (&SoftBodyPoint, &mut Transform),
        (Without<ParticleBody>, Without<RigidBody>),
    >,
) {
    let physics_world = &mut physics_world.world;

    physics_world.apply_effectors();
    physics_world.step(time.delta_secs_f64());
    physics_world.handle_collisions();

    for (body, mut transform) in particle_query.iter_mut() {
        if let ParticleBody(BodyId::Particle(id)) = body {
            let position = physics_world.get_linear(*id).unwrap().position;
            transform.translation.x = position.x as f32;
            transform.translation.y = position.y as f32;
        }
    }

    for (body, mut transform) in rigid_body_query.iter_mut() {
        if let RigidBody(BodyId::Rigid(id)) = body {
            let position = physics_world.get_linear(*id).unwrap().position;
            transform.translation.x = position.x as f32;
            transform.translation.y = position.y as f32;

            let rotation = physics_world.get_angular(*id).unwrap().rotation;
            transform.rotation = Quat::from_rotation_z(-rotation as f32);
        }
    }

    // soft_body_children_query
    //     .iter_mut()
    //     .for_each(|(SoftBody(BodyId::Soft { points, springs }), children)| {});
}

fn update() {}
