use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use i_triangle::float::triangulatable::Triangulatable;
use physics::{
    object::{Object, Shape},
    types::Vec2 as PhysicsVec2,
    world::World,
};

const POINT_SIZE: f32 = 3.0;

#[derive(Resource)]
struct PhysicsWorld {
    world: World,
}

#[derive(Component)]
struct PhysicsObject {
    // Switch from usize to some handle
    id: usize,
}

fn spawn_physics_object(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    physics_world: &mut ResMut<PhysicsWorld>,
    position: Vec2,
    velocity: Vec2,
    mass: f64,
    shape: Shape,
    colour: Color,
) {
    let mesh: Mesh = match &shape {
        Shape::Point => Circle::new(POINT_SIZE).into(),
        Shape::Circle(radius) => Circle::new(radius.clone() as f32).into(),
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
    };

    let physics_object = Object::new(
        PhysicsVec2::new(position.x as f64, position.y as f64),
        PhysicsVec2::new(velocity.x as f64, velocity.y as f64),
        mass,
        shape,
    );

    let physics_id = physics_world.world.add(physics_object);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(colour)),
        Transform::from_xyz(position.x, position.y, 0.0),
        PhysicsObject { id: physics_id },
    ));
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

    spawn_physics_object(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Vec2::new(0.0, 100.0),
        Vec2::new(100.0, 0.0),
        4000000.0,
        Shape::Circle(100.0),
        Color::linear_rgb(0.0, 1.0, 0.0),
    );

    spawn_physics_object(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut physics_world,
        Vec2::new(0.0, -100.0),
        Vec2::new(-100.0, 0.0),
        4000000.0,
        Shape::Circle(100.0),
        Color::linear_rgb(1.0, 0.0, 0.0),
    );
}

fn update_physics(
    mut physics_world: ResMut<PhysicsWorld>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &PhysicsObject)>,
) {
    let physics_world = &mut physics_world.world;

    physics_world.step(time.delta_secs_f64());

    query.iter_mut().for_each(|(mut transform, physics)| {
        let position = physics_world.get(physics.id).unwrap().position;
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
    });
}

fn update() {}
