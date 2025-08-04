use bevy::prelude::*;
use i_triangle::float::triangulatable::Triangulatable;
use physics::{
    Vector2,
    object::{Object, Shape},
    world::World,
};

const POINT_SIZE: f32 = 3.0;

#[derive(Resource)]
struct PhysicsWorld {
    world: World,
}

#[derive(Resource)]
struct PhysicsObjects {
    objects: Vec<usize>,
}

#[derive(Component)]
struct PhysicsObject {
    // Switch from usize to some handle
    id: usize,
}

fn spawn_physics_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut physics_world: ResMut<PhysicsWorld>,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f64,
    shape: Shape,
    colour: Color,
) {
    let physics_object = Object::new(
        Vector2::new(position.x as f64, position.y as f64),
        Vector2::new(velocity.x as f64, velocity.y as f64),
        Vector2::new(acceleration.x as f64, acceleration.y as f64),
        mass,
        shape,
    );

    let physics_id = physics_world.world.add(physics_object);

    let mesh = match shape {
        Shape::Point => Circle::new(POINT_SIZE),
        Shape::Circle(radius) => Circle::new(radius as f32),
        Shape::Polygon(points) => {
            let shape: Vec<[f64; 2]> = points.iter().map(|p| [p.x, p.y]).collect();
            shape.triangulate().to_triangulation();
        }
    };

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(3.0))),
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
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    physics_world: ResMut<PhysicsWorld>,
) {
    commands.spawn(Camera2d);

    spawn_physics_object(
        commands,
        meshes,
        materials,
        physics_world,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, -10.0),
        1.0,
        Shape::Point,
        Color::linear_rgb(0.0, 1.0, 0.0),
    );
}

fn update_physics(
    mut physics_world: ResMut<PhysicsWorld>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &PhysicsObject)>,
) {
    let physics_world = &mut physics_world.world;

    query.iter_mut().for_each(|(mut transform, physics)| {
        let position = physics_world.get(physics.id).unwrap().position;
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
    });

    physics_world.step(time.delta_secs_f64());
}

fn update() {}
