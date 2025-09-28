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
    effector::{ConstantAcceleration, Drag, Gravity, Spring},
    id_map::{Id, IdMap},
    integrator::SemiImplicitEuler,
    soft_body::{self, SoftBodyId, SoftBodySpring},
    types::math::*,
    world::World,
};

use crate::scenes::{self, PhysicsScene, falling_rectangles};

pub const POINT_SIZE: f32 = 10.0;
pub const SPRING_SIZE: f32 = 10.0;

#[derive(Resource)]
pub struct PhysicsWorld {
    pub world: World,
}

// Make enum?
#[derive(Component)]
pub struct BodyId(pub Id);

#[derive(Component)]
pub struct EffectorId(pub Id);

#[derive(Component)]
pub struct SpringEffector;

pub fn shape_to_mesh(shape: &Shape) -> Mesh {
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

pub fn spawn_physics_body(
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
        .with_rotation(Quat::from_rotation_z(body.angular.orientation as f32)),
        BodyId(id),
    ));

    id
}

pub fn spawn_physics_soft_body(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    physics_world: &mut ResMut<PhysicsWorld>,
    points: Vec<LinearState>,
    springs: Vec<SoftBodySpring>,
    colour: Color,
) -> SoftBodyId {
    let id = physics_world
        .world
        .add_soft_body(points.clone(), springs.clone());
    let SoftBodyId {
        points: point_ids,
        springs: spring_ids,
    } = &id;

    for (linear, id) in points.iter().zip(point_ids) {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(POINT_SIZE))),
            MeshMaterial2d(materials.add(colour)),
            Transform::from_xyz(linear.position.x as f32, linear.position.y as f32, 0.0),
            BodyId(*id),
        ));
    }

    for (spring, id) in springs.iter().zip(spring_ids) {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1.0, SPRING_SIZE))),
            MeshMaterial2d(materials.add(colour)),
            Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(0.0)),
            EffectorId(*id),
            SpringEffector,
        ));
    }

    id
}

pub fn reset_physics_world(
    commands: &mut Commands,
    physics_entities: Query<Entity, Or<(With<BodyId>, With<EffectorId>)>>,
    physics_world: &mut ResMut<PhysicsWorld>,
) {
    for entity in &physics_entities {
        commands.entity(entity).despawn();
    }

    physics_world.world.reset();
}

#[derive(Event)]
pub struct LoadSceneEvent(pub PhysicsScene);

pub fn handle_load_scene(
    mut load_event: EventReader<LoadSceneEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    physics_entities: Query<Entity, Or<(With<BodyId>, With<EffectorId>)>>,
    mut physics_world: ResMut<PhysicsWorld>,
) {
    for LoadSceneEvent(scene) in load_event.read() {
        reset_physics_world(&mut commands, physics_entities, &mut physics_world);

        match scene {
            PhysicsScene::FallingRectangles => scenes::falling_rectangles::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
            PhysicsScene::Tower => scenes::tower::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
            PhysicsScene::FallingCircles => scenes::falling_circles::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
            PhysicsScene::CircleCollision => scenes::circle_collision::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
            PhysicsScene::Spring => scenes::spring::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
            PhysicsScene::CollisionSpring => scenes::collision_spring::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
            PhysicsScene::Polygon => scenes::polygon::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
            PhysicsScene::Orbit => scenes::orbit::load(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut physics_world,
            ),
        }
    }
}
