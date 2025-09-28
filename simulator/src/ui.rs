use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::physics_helpers::{
    BodyId, EffectorId, LoadSceneEvent, PhysicsWorld, reset_physics_world,
};
use crate::scenes::{self, PhysicsScene};

#[derive(Default, Resource)]
pub struct UiState {
    pub is_intro_open: bool,
}

pub fn ui_pass(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut load_event: EventWriter<LoadSceneEvent>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Intro")
        .open(&mut ui_state.is_intro_open)
        .show(ctx, |ui| {
            ui.label(
                "Hello welcome to my physics engine. \n\
            I have made some demo scenes that you can load on the left. \n\
            You can control the camera with WASD and scroll wheel. \n\
            Sadly you cannot interact with the scene as I ran out of time. \n\
            Thank you for trying out my engine hope you like it :)",
            )
        });

    egui::SidePanel::left("left_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Scenes");

            if ui.button("Falling Rectangles").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::FallingRectangles));
            }

            if ui.button("Falling Circles").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::FallingCircles));
            }

            if ui.button("Tower").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::Tower));
            }

            if ui.button("Circle Collision").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::CircleCollision));
            }

            if ui.button("Spring").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::Spring));
            }

            if ui.button("Collision Spring").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::CollisionSpring));
            }

            if ui.button("Polygon").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::Polygon));
            }

            if ui.button("Orbit").clicked() {
                load_event.write(LoadSceneEvent(PhysicsScene::Orbit));
            }
        });

    Ok(())
}
