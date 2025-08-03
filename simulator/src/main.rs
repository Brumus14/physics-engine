use bevy::prelude::*;
use physics::{
    Vector2,
    object::{Object, Shape},
    world::World,
};

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut physics_world = World::new();

    let object = Object::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, -100.0),
        1.0,
        Shape::Point,
    );

    let object_id = physics_world.add(object);

    commands.insert_resource(PhysicsWorld {
        world: physics_world,
    });

    let shape = meshes.add(Circle::new(50.0));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        PhysicsObject { id: object_id },
    ));
}

fn update(
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

    println!(
        "Time: {}, New position: {:?}",
        time.elapsed().as_secs_f64(),
        physics_world.get(0).unwrap().position
    );
}
