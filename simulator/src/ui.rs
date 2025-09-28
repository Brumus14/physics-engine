use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::physics_helpers::{
    BodyId, EffectorId, LoadSceneEvent, PhysicsScene, PhysicsWorld, ResetSceneEvent,
    reset_physics_world,
};
use crate::scenes;

#[derive(Default, Resource)]
pub struct UiState {
    pub is_intro_open: bool,
}

pub fn ui_pass(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut physics_world: ResMut<PhysicsWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    physics_entities: Query<Entity, Or<(With<BodyId>, With<EffectorId>)>>,
    mut reset_event: EventWriter<ResetSceneEvent>,
    mut load_event: EventWriter<LoadSceneEvent>,
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
                reset_event.write(ResetSceneEvent);
                load_event.write(LoadSceneEvent(PhysicsScene::FallingRectangles));
            }
        });

    Ok(())
}
