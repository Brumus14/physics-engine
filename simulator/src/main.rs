mod camera_controller;
mod physics_helpers;
mod scenes;

use crate::camera_controller::camera_controller;
use crate::physics_helpers::*;
use bevy::{
    asset::RenderAssetUsages,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::mouse::MouseWheel,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    text::FontSmoothing,
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
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
use rand::Rng;
use std::f64;

fn main() {
    App::new()
        .init_resource::<UiState>()
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
            EguiPlugin::default(),
        ))
        .add_systems(Startup, (startup_physics, startup).chain())
        .add_systems(Update, (handle_input, update_physics, update).chain())
        .add_systems(Update, camera_controller)
        .add_systems(EguiPrimaryContextPass, ui_pass)
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
    mut ui_state: ResMut<UiState>,
) {
    ui_state.is_intro_open = true;

    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let mut rng = rand::rng();

    let mut bodies: Vec<Id> = Vec::new();

    for i in 0..100 {
        bodies.push(spawn_physics_body(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut physics_world,
            Body::new_rigid(
                LinearState::new(
                    Vector::new(
                        rng.random_range(-800.0..800.0),
                        rng.random_range(-500.0..500.0),
                    ),
                    Vector::zeros(),
                    1.0,
                ),
                1.0,
                AngularState::new(rng.random_range(0.0..f64::consts::TAU), 0.0, 1000.0),
                Shape::new_circle(25.0),
            ),
            Color::WHITE,
        ));
    }
}

fn update_physics(
    mut physics_world: ResMut<PhysicsWorld>,
    time: Res<Time>,
    mut body_query: Query<(&BodyId, &mut Transform), Without<EffectorId>>,
    mut spring_query: Query<(&EffectorId, &mut Transform), (With<SpringEffector>, Without<BodyId>)>,
) {
    let physics_world = &mut physics_world.world;

    physics_world.apply_effectors();
    physics_world.step(time.delta_secs_f64());
    physics_world.handle_collisions();

    for (body_id, mut transform) in body_query.iter_mut() {
        let BodyId(id) = body_id;

        let position = physics_world.get_body(*id).unwrap().linear.position;
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;

        let rotation = physics_world.get_body(*id).unwrap().angular.orientation;
        transform.rotation = Quat::from_rotation_z(rotation as f32);
    }

    for (effector_id, mut transform) in spring_query.iter_mut() {
        let EffectorId(id) = effector_id;

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

fn handle_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut physics_world: ResMut<PhysicsWorld>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = window.single().unwrap();
        let (camera, transform) = camera.single().unwrap();

        if let Some(position) = window.cursor_position() {
            if let Ok(position) = camera.viewport_to_world_2d(transform, position) {
                spawn_physics_body(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut physics_world,
                    Body::new_rigid(
                        LinearState::new(
                            Vector::new(position.x as f64, position.y as f64),
                            Vector::zeros(),
                            1.0,
                        ),
                        1.0,
                        AngularState::new(0.0, 0.0, 1.0),
                        Shape::Circle(25.0),
                    ),
                    Color::WHITE,
                );
            }
        }
    }
}

#[derive(Default, Resource)]
struct UiState {
    is_intro_open: bool,
}

fn ui_pass(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut physics_world: ResMut<PhysicsWorld>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Intro")
        .open(&mut ui_state.is_intro_open)
        .show(ctx, |ui| ui.label("Welcome to my physics engine"));

    egui::SidePanel::left("left_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Manager");

            if ui.button("Falling Rectangles").clicked() {
                scenes::falling_rectangles(&mut physics_world.world);
            }
        });
    Ok(())
}

fn update() {}
