use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{components::Player, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, spawn_player)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(rotate_player)
                    .into(),
            );
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Collider::cuboid(32.0, 32.0))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn rotate_player(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let (camera, camera_transform) = camera.single();

    let window = windows.primary();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let mouse_pos: Vec2 = world_pos.truncate();

        let mut player_transform = player.single_mut();

        // normalized vector pointing from player to mouse
        let dir = (mouse_pos - player_transform.translation.truncate()).normalize();

        player_transform.rotation = Quat::from_rotation_z(if dir.x < 0.0 {
            Vec2::Y.angle_between(dir)
        } else {
            Vec2::NEG_Y.angle_between(dir)
        });
    }
}
