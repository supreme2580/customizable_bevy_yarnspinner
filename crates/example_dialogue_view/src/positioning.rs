use crate::config::DialogueViewConfig;
use crate::setup::UiRootNode;
use bevy::prelude::*;
use bevy::render::camera::Camera;

/// Component for 3D positioned dialogue
#[derive(Component)]
pub struct Dialogue3DPosition {
    pub world_position: Vec3,
    pub offset: Vec2,
}

/// System to position dialogue in 3D space
pub fn position_dialogue_3d(
    mut dialogue_queries: Query<(&mut Transform, &Dialogue3DPosition), With<UiRootNode>>,
    camera_queries: Query<(&Camera, &GlobalTransform), Without<UiRootNode>>,
    windows: Query<&Window>,
    config: Res<DialogueViewConfig>,
) {
    // Only run if 3D positioning is enabled
    if !config.use_3d_positioning {
        return;
    }

    let window_result = windows.single();
    let camera_result = camera_queries.single();

    if let (Ok(window), Ok((camera, camera_transform))) = (window_result, camera_result) {
        for (mut transform, dialogue_pos) in dialogue_queries.iter_mut() {
            // Convert 3D world position to screen position
            if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, dialogue_pos.world_position) {
                // Convert from viewport coordinates (0-1) to screen coordinates
                let screen_x = (screen_pos.x * window.width() as f32) + dialogue_pos.offset.x;
                let screen_y = ((1.0 - screen_pos.y) * window.height() as f32) + dialogue_pos.offset.y;

                // Update the UI transform
                transform.translation = Vec3::new(screen_x, screen_y, 0.0);
            }
        }
    }
}

/// System to update dialogue position when camera moves
pub fn update_dialogue_position(
    _dialogue_queries: Query<&mut Transform, (With<UiRootNode>, With<Dialogue3DPosition>)>,
    _camera_queries: Query<&GlobalTransform, (With<Camera>, Without<UiRootNode>)>,
    _windows: Query<&Window>,
) {
    // This system will be called when the camera moves
    // The actual positioning logic is in position_dialogue_3d
}

/// Helper function to create a 3D positioned dialogue
pub fn create_3d_dialogue_position(world_position: Vec3, screen_offset: Vec2) -> Dialogue3DPosition {
    Dialogue3DPosition {
        world_position,
        offset: screen_offset,
    }
}
