use std::f64;

use bevy::{
    asset::RenderAssetUsages,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    text::FontSmoothing,
};
use i_triangle::float::triangulatable::Triangulatable;
use physics::{
    body::{AngularState, Body, LinearState, Shape},
    collision::default::{DefaultCollisionPipeline, DefaultNarrowPhase},
    effector::{ConstantAcceleration, Drag, Spring},
    id_map::{Id, IdMap},
    integrator::SemiImplicitEuler,
    soft_body::{self, SoftBodySpring},
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

// Make enum?
#[derive(Component)]
struct BodyId(Id);

#[derive(Component)]
struct SpringId(Id);

fn shape_to_mesh(shape: &Shape) -> Mesh {
    match shape {
        Shape::Point => Circle::new(POINT_SIZE).into(),
        Shape::Circle(radius) => Circle::new(radius.clone() as f32).into(),
        Shape::Polygon { points, axes: _ } => {
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

fn spawn_physics_body(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    physics_world: &mut ResMut<PhysicsWorld>,
    body: Body,
    colour: Color,
) -> Id {
    let id = physics_world.world.add_body(body.clone());

    commands.spawn((
        Mesh2d(meshes.add(shape_to_mesh(&body.shape))),
        MeshMaterial2d(materials.add(colour)),
        Transform::from_xyz(
            body.linear.position.x as f32,
            body.linear.position.y as f32,
            0.0,
        )
        .with_rotation(Quat::from_rotation_z(-body.angular.orientation as f32)),
        BodyId(id),
    ));

    id
}

// fn spawn_physics_soft_body(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<ColorMaterial>>,
//     physics_world: &mut ResMut<PhysicsWorld>,
//     points: Vec<LinearState>,
//     springs: Vec<SoftBodySpring>,
//     colour: Color,
// ) -> Id {
//     let id = physics_world
//         .world
//         .add_soft_body(points.clone(), springs.clone());
//     let group = physics_world.world.get_body_group(id).unwrap();
//     let point_ids = physics_world.world.get_body_group(group[0]).unwrap();
//     let spring_ids = physics_world.world.get_body_group(group[1]).unwrap();
//
//     for (linear, id) in points.iter().zip(point_ids) {
//         commands.spawn((
//             Mesh2d(meshes.add(Circle::new(POINT_SIZE))),
//             MeshMaterial2d(materials.add(colour)),
//             Transform::from_xyz(linear.position.x as f32, linear.position.y as f32, 0.0),
//             ParticleBodyId(*id),
//         ));
//     }
//
//     for (spring, id) in springs.iter().zip(spring_ids) {
//         commands.spawn((
//             Mesh2d(meshes.add(Rectangle::new(1.0, SPRING_SIZE))),
//             MeshMaterial2d(materials.add(colour)),
//             Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(0.0)),
//             SpringId(*id),
//         ));
//     }
//
//     id
// }

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 42.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    text_color: Color::WHITE,
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                },
            },
        ))
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

    let mut rng = rand::rng();

    let mut bodies: Vec<Id> = Vec::new();

    // for i in 0..100 {
    //     bodies.push(spawn_physics_body(
    //         &mut commands,
    //         &mut meshes,
    //         &mut materials,
    //         &mut physics_world,
    //         Body::new_rigid(
    //             LinearState::new(
    //                 Vector::new(
    //                     rng.random_range(-800.0..800.0),
    //                     rng.random_range(-500.0..500.0),
    //                 ),
    //                 Vector::zeros(),
    //                 1.0,
    //             ),
    //             1.0,
    //             AngularState::new(rng.random_range(0.0..f64::consts::TAU), 0.0, 1.0),
    //             Shape::new_rectangle(Vector::new(100.0, 50.0)),
    //         ),
    //         Color::WHITE,
    //     ));
    // }

    bodies.push(spawn_physics_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Body::new_rigid(
            LinearState::new(Vector::new(0.0, 200.0), Vector::zeros(), 1.0),
            1.0,
            AngularState::new(0.2, 0.0, 1.0),
            Shape::new_rectangle(Vector::new(100.0, 50.0)),
        ),
        Color::WHITE,
    ));

    bodies.push(spawn_physics_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Body::new_rigid(
            LinearState::new(Vector::new(0.0, 0.0), Vector::zeros(), f64::INFINITY),
            1.0,
            AngularState::new(0.0, 0.0, 1.0),
            Shape::new_rectangle(Vector::new(100.0, 50.0)),
        ),
        Color::WHITE,
    ));

    // let ground = spawn_physics_body(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     &mut physics_world,
    //     Body::new_rigid(
    //         LinearState::new(Vector::new(0.0, -500.0), Vector::zeros(), f64::INFINITY),
    //         0.8,
    //         AngularState::new(0.0, 0.0, 1.0),
    //         Shape::new_rectangle(Vector::new(1600.0, 50.0)),
    //     ),
    //     Color::WHITE,
    // );

    physics_world
        .world
        .add_integrator(Box::new(SemiImplicitEuler::new(bodies.clone())));

    physics_world
        .world
        .add_collision_pipeline(Box::new(DefaultCollisionPipeline::new(
            // vec![bodies.as_slice(), &[ground]].concat(),
            bodies.clone(),
        )));

    physics_world
        .world
        .add_effector(Box::new(ConstantAcceleration::new(
            vec![bodies[0]],
            Vector::new(0.0, -200.0),
        )));
}

fn update_physics(
    mut physics_world: ResMut<PhysicsWorld>,
    time: Res<Time>,
    mut body_query: Query<(&BodyId, &mut Transform), Without<SpringId>>,
    mut spring_query: Query<(&SpringId, &mut Transform), Without<BodyId>>,
) {
    let physics_world = &mut physics_world.world;

    physics_world.apply_effectors();
    physics_world.step(time.delta_secs_f64());
    physics_world.handle_collisions();

    for (body, mut transform) in body_query.iter_mut() {
        let BodyId(id) = body;

        let position = physics_world.get_body(*id).unwrap().linear.position;
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;

        let rotation = physics_world.get_body(*id).unwrap().angular.orientation;
        transform.rotation = Quat::from_rotation_z(-rotation as f32);
    }

    for (body, mut transform) in spring_query.iter_mut() {
        let SpringId(id) = body;

        let spring = physics_world
            .get_effector(*id)
            .unwrap()
            // Convert this into a function like as<Spring>
            .as_any()
            .downcast_ref::<Spring>()
            .unwrap();
        let [a_id, b_id] = spring.bodies;
        let (a_position, b_position) = (
            physics_world.get_body(a_id).unwrap().linear.position,
            physics_world.get_body(b_id).unwrap().linear.position,
        );

        let length = a_position.metric_distance(&b_position);
        let mut rotation = (b_position.y - a_position.y).atan2(b_position.x - a_position.x);

        if rotation < 0.0 {
            rotation += std::f64::consts::TAU;
        }

        let position = a_position + (b_position - a_position) / 2.0;

        transform.scale.x = length as f32;
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
        transform.rotation = Quat::from_rotation_z(rotation as f32);
    }
}

fn update() {}
