use bevy::{input::mouse::MouseWheel, prelude::*};

pub fn camera_controller(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut transform = camera.single_mut().unwrap();

    for event in mouse_wheel_events.read() {
        if event.y > 0.0 {
            transform.scale *= 0.96;
        }

        if event.y < 0.0 {
            transform.scale *= 1.04;
        }
    }

    let mut movement_direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {
        movement_direction.y += 1.0;
    }

    if input.pressed(KeyCode::KeyS) {
        movement_direction.y -= 1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        movement_direction.x += 1.0;
    }

    if input.pressed(KeyCode::KeyA) {
        movement_direction.x -= 1.0;
    }

    if movement_direction.length() != 0.0 {
        let movement_direction = movement_direction.normalize() * 6.0 * transform.scale.y;
        transform.translation += Vec3::new(movement_direction.x, movement_direction.y, 0.0);
    }
}
