use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{AnimationTimer, Barrel, Bullet, Enemy, Player, Ready},
    resources::{MousePosition, Sprites},
    GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .add_enter_system(GameState::Playing, spawn_player)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(update_mouse_position)
                    .with_system(shoot)
                    .with_system(rotate_player)
                    .with_system(animate_player)
                    .with_system(collide_bullets)
                    .with_system(despawn_offscreen)
                    .into(),
            );
    }
}

fn spawn_player(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(5.)),
            texture: sprites.base.clone(),
            ..default()
        })
        .insert(Player)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(7., 7.))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprites.barrel.clone(),
                    ..default()
                })
                .insert(Barrel)
                .insert(AnimationTimer(Timer::from_seconds(0.125, true)))
                .insert(Ready(false));
        });
}

fn shoot(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePosition>,
    mut player: Query<&Transform, With<Player>>,
    mut barrel: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Ready), With<Barrel>>,
) {
    let transform = player.single_mut();
    let (mut sprite, mut timer, mut ready) = barrel.single_mut();

    if mouse_buttons.just_pressed(MouseButton::Left) && timer.paused() {
        timer.unpause();
        ready.0 = false;
        sprite.index = 1;
    }

    if timer.just_finished() && !timer.paused() && sprite.index == 0 {
        let dir = (mouse_pos.0 - transform.translation.truncate()).normalize();

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_translation(transform.translation).with_scale(Vec3::splat(24.0)),
                ..default()
            })
            .insert(Bullet)
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Velocity::linear(dir * 1500.0))
            .insert(Collider::cuboid(0.5, 0.5))
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }

    if !mouse_buttons.pressed(MouseButton::Left) && sprite.index == 0 {
        ready.0 = true;
    }
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut barrel: Query<
        (
            &mut AnimationTimer,
            &Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
            &mut Ready,
        ),
        With<Barrel>,
    >,
) {
    let (mut anim_timer, atlas_handle, mut sprite, mut ready) = barrel.single_mut();

    if anim_timer.paused() {
        anim_timer.set_elapsed(Duration::ZERO);
        sprite.index = 0;
    } else {
        anim_timer.tick(time.delta());

        if anim_timer.just_finished() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();

            sprite.index = (sprite.index + 1) % texture_atlas.len();

            if ready.0 && sprite.index == 1 {
                sprite.index = 0;
                anim_timer.pause();
                ready.0 = false;
            }
        }
    }
}

fn rotate_player(
    mouse_pos: Res<MousePosition>,
    mut barrel: Query<(&mut Transform, &mut TextureAtlasSprite), With<Barrel>>,
) {
    let (mut player_transform, mut sprite) = barrel.single_mut();

    // normalized vector pointing from player to mouse
    let dir = (mouse_pos.0 - player_transform.translation.truncate()).normalize();

    player_transform.rotation = Quat::from_rotation_z(if dir.x < 0.0 {
        sprite.flip_x = false;
        sprite.flip_y = false;
        Vec2::Y.angle_between(dir)
    } else {
        sprite.flip_x = true;
        sprite.flip_y = true;
        Vec2::NEG_Y.angle_between(dir)
    });
}

fn collide_bullets(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut handled_entities = HashSet::new();

    for ev in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = ev {
            if handled_entities.contains(e1) || handled_entities.contains(e2) {
                continue;
            }

            let (&bullet_entity, maybe_enemy) = if let Ok(_) = bullets.get(*e1) {
                (e1, e2)
            } else if let Ok(_) = bullets.get(*e2) {
                (e2, e1)
            } else {
                continue;
            };

            if let Ok(enemy_entity) = enemies.get(*maybe_enemy) {
                commands.entity(bullet_entity).despawn_recursive();
                commands.entity(enemy_entity).despawn_recursive();
                handled_entities.insert(bullet_entity);
                handled_entities.insert(enemy_entity);
            }
        }
    }
}

fn despawn_offscreen(
    mut commands: Commands,
    windows: Res<Windows>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
) {
    let window = windows.primary();

    for (entity, transform) in &bullets {
        if transform.translation.x > window.width()
            || transform.translation.x < -window.width()
            || transform.translation.y > window.height()
            || transform.translation.y < -window.height()
        {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn update_mouse_position(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut mouse_pos: ResMut<MousePosition>,
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
        *mouse_pos = MousePosition(world_pos.truncate());
    }
}
