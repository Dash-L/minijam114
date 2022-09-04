use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{
        Barrel, Bullet, Coin, Damage, Enemy, Health, HitEnemies, ImmobileTimer, Knockback, Pierce,
        Player, Ready,
    },
    resources::{
        BulletType, Coins, Fonts, HasIce, HasSuck, MousePosition, ShootTimer, Spread, Sprites,
    },
    GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .init_resource::<Coins>()
            .init_resource::<Spread>()
            .init_resource::<BulletType>()
            .init_resource::<HasIce>()
            .init_resource::<HasSuck>()
            .insert_resource(Damage(50.0))
            .insert_resource(Knockback(0.0))
            .insert_resource(Pierce(1))
            .insert_resource(ShootTimer(Timer::from_seconds(0.125, true)))
            .add_exit_system(GameState::Menu, spawn_player)
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

fn spawn_player(mut commands: Commands, fonts: Res<Fonts>, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(5.))
                .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            texture: sprites.base.clone(),
            ..default()
        })
        .insert(Player)
        .insert(Health::new(200.0))
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
                .insert(Ready(false));
        });

    commands
        .spawn_bundle(NodeBundle {
            color: UiColor([0.0; 4].into()),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(300.0), Val::Px(20.0)),
                margin: UiRect {
                    bottom: Val::Auto,
                    ..default()
                },
                position: UiRect {
                    top: Val::Px(15.0),
                    left: Val::Px(130.0),
                    ..default()
                },
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                image: UiImage(sprites.coin.clone()),
                ..default()
            });
            parent
                .spawn_bundle(TextBundle::from_section(
                    "0",
                    TextStyle {
                        color: Color::YELLOW,
                        font: fonts.main.clone(),
                        font_size: 30.0,
                    },
                ))
                .insert(Coin);
        });
}

fn shoot(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    sprites: Res<Sprites>,
    mouse_pos: Res<MousePosition>,
    knockback: Res<Knockback>,
    bullet_type: Res<BulletType>,
    spread: Res<Spread>,
    damage: Res<Damage>,
    pierce: Res<Pierce>,
    mut timer: ResMut<ShootTimer>,
    mut player: Query<&Transform, With<Player>>,
    mut barrel: Query<(&mut TextureAtlasSprite, &mut Ready), With<Barrel>>,
) {
    let transform = player.single_mut();
    let (mut sprite, mut ready) = barrel.single_mut();

    if mouse_buttons.just_pressed(MouseButton::Left) && timer.paused() {
        timer.unpause();
        ready.0 = false;
        sprite.index = 1;
    }

    let mut rng = thread_rng();

    if timer.just_finished() && !timer.paused() && sprite.index == 0 {
        let dir = Vec2::from_angle(rng.gen_range(-spread.1..=spread.1))
            .rotate(mouse_pos.0 - transform.translation.truncate())
            .normalize();

        let scale = match *bullet_type {
            BulletType::Regular => Vec3::splat(1.5),
            BulletType::Rocket => Vec3::splat(2.5),
            BulletType::SawBlade => Vec3::splat(2.0),
        };

        commands
            .spawn_bundle(SpriteBundle {
                texture: match *bullet_type {
                    BulletType::Regular => sprites.bullet.clone(),
                    BulletType::Rocket => sprites.rocket.clone(),
                    BulletType::SawBlade => sprites.saw_blade.clone(),
                },
                transform: Transform::from_translation(transform.translation)
                    .with_scale(scale)
                    .with_rotation(Quat::from_rotation_z(Vec2::X.angle_between(dir))),
                ..default()
            })
            .insert(Bullet)
            .insert(pierce.clone())
            .insert(damage.clone())
            .insert(knockback.clone())
            .insert(HitEnemies::default())
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Velocity::linear(dir * 1500.0))
            .insert(Collider::cuboid(8.0, 8.0))
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
    mut anim_timer: ResMut<ShootTimer>,
    mut barrel: Query<(&Handle<TextureAtlas>, &mut TextureAtlasSprite, &mut Ready), With<Barrel>>,
) {
    let (atlas_handle, mut sprite, mut ready) = barrel.single_mut();

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

    player_transform.rotation = Quat::from_rotation_z(
        if dir.x < 0.0 {
            sprite.flip_x = false;
            sprite.flip_y = false;
            Vec2::Y.angle_between(dir)
        } else {
            sprite.flip_x = true;
            sprite.flip_y = true;
            Vec2::NEG_Y.angle_between(dir)
        } - PI / 2.0,
    );
}

fn collide_bullets(
    mut commands: Commands,
    has_ice: Res<HasIce>,
    mut bullets: Query<(Entity, &mut HitEnemies, &mut Pierce, &Damage, &Knockback), With<Bullet>>,
    mut enemies: Query<
        (
            Entity,
            Option<&ImmobileTimer>,
            &ExternalForce,
            &mut Health,
            &mut ExternalImpulse,
        ),
        With<Enemy>,
    >,
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

            let (_, mut hit_enemies, mut pierce, damage, knockback) =
                bullets.get_mut(bullet_entity).unwrap();
            if let Ok((enemy_entity, maybe_immobile, force, mut health, mut impulse)) =
                enemies.get_mut(*maybe_enemy) && !hit_enemies.0.contains(&enemy_entity)
            {
                hit_enemies.0.insert(enemy_entity);
                pierce.0 -= 1;
                impulse.impulse = force.force.normalize() * -knockback.0;
                if has_ice.0 && maybe_immobile.is_none() {
                    commands.entity(enemy_entity).insert(ImmobileTimer(Timer::from_seconds(0.25, false)));
                }
                if pierce.0 <= 0 {
                    commands.entity(bullet_entity).despawn_recursive();
                }
                health.0 -= damage.0;
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
