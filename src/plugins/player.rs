use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{AnimationTimer, Bullet, Enemy, FireRate, LastShotTime, Player},
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
                    .with_system(update_animation_timer)
                    .with_system(animate_player)
                    .with_system(collide_bullets)
                    .with_system(despawn_offscreen)
                    .into(),
            );
    }
}

fn spawn_player(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            texture_atlas: sprites.player.clone(),
            ..default()
        })
        .insert(FireRate(0.25))
        .insert(LastShotTime::default())
        .insert(AnimationTimer(Timer::from_seconds(0.25, true)))
        .insert(Player)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(32.0, 32.0))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn shoot(
    mut commands: Commands,
    time: Res<Time>,
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePosition>,
    mut player: Query<
        (
            &Transform,
            &FireRate,
            &mut LastShotTime,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
) {
    let (transform, fire_rate, mut last_shot_time, mut timer) = player.single_mut();

    last_shot_time.tick(time.delta());

    if mouse_buttons.pressed(MouseButton::Left) {
        timer.unpause();
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let timer_duration = timer.duration();
            timer.set_elapsed(timer_duration);
        }

        if last_shot_time.elapsed().as_secs_f32() > fire_rate.0 {
            last_shot_time.reset();
            let dir = (mouse_pos.0 - transform.translation.truncate()).normalize();

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::YELLOW,
                        custom_size: Some(Vec2::new(6.0, 6.0)),
                        ..default()
                    },
                    transform: transform.clone(),
                    ..default()
                })
                .insert(Bullet)
                .insert(RigidBody::Dynamic)
                .insert(Ccd::enabled())
                .insert(Velocity::linear(dir * 1500.0))
                .insert(Collider::cuboid(3.0, 3.0))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS);
        }
    } else {
        timer.pause()
    }
}

fn update_animation_timer(
    mut player: Query<(&mut AnimationTimer, &FireRate), (With<Player>, Changed<FireRate>)>,
) {
    if let Ok((mut anim_timer, fire_rate)) = player.get_single_mut() {
        anim_timer.set_duration(Duration::from_secs_f32(fire_rate.0));
    }
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player: Query<
        (
            &mut AnimationTimer,
            &Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    let (mut anim_timer, atlas_handle, mut sprite) = player.single_mut();

    if anim_timer.paused() {
        anim_timer.set_elapsed(Duration::ZERO);
        sprite.index = 0;
    } else {
        anim_timer.tick(time.delta());

        if anim_timer.just_finished() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();

            sprite.index += 1;
            if sprite.index >= texture_atlas.len() {
                sprite.index = 1;
            }
        }
    }
}

fn rotate_player(
    mouse_pos: Res<MousePosition>,
    mut player: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
) {
    let (mut player_transform, mut sprite) = player.single_mut();

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
