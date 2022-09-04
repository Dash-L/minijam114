use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use rand::{distributions::Standard, prelude::*};

use crate::{
    components::{AttackTimer, Damage, Enemy, Health, Player},
    resources::{EnemyScale, ScaleTimer, Sounds, Sprites, SpawnTimer},
    GameState,
};

pub struct EnemyPlugin;
pub enum EnemySpawnPos {
    Up,
    Down,
    Left,
    Right,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>()
            .init_resource::<EnemyScale>()
            .insert_resource(ScaleTimer(Timer::from_seconds(1.0, true)))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(spawn_enemies)
                    .with_system(scale_enemies)
                    .with_system(move_to_player)
                    .with_system(damage_player)
                    .into(),
            );
    }
}

impl Distribution<EnemySpawnPos> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemySpawnPos {
        let val = rng.gen_range(0..4);
        match val {
            0 => EnemySpawnPos::Up,
            1 => EnemySpawnPos::Down,
            2 => EnemySpawnPos::Left,
            3 => EnemySpawnPos::Right,
            _ => unreachable!(),
        }
    }
}
fn scale_enemies(
    time: Res<Time>,
    mut enemy_scale: ResMut<EnemyScale>,
    mut scale_timer: ResMut<ScaleTimer>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    scale_timer.tick(time.delta());
    if scale_timer.just_finished() {
        enemy_scale.0 *= 1.005;
        let duration = spawn_timer.duration();
        spawn_timer.set_duration(duration.div_f32(1.01));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    enemy_scale: Res<EnemyScale>,
    mut spawn_timer: ResMut<SpawnTimer>,
    windows: Res<Windows>,
    sprites: Res<Sprites>,
) {
    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() {
        let window = windows.primary();

        let mut rng = rand::thread_rng();
        let enemy_pos: EnemySpawnPos = rng.gen();

        let translation = match enemy_pos {
            EnemySpawnPos::Up => Vec3::new(
                rng.gen_range((-window.width() / 2.0)..(window.width() / 2.0)),
                -window.height() / 2.,
                1.,
            ),
            EnemySpawnPos::Down => Vec3::new(
                rng.gen_range((-window.width() / 2.0)..(window.width() / 2.0)),
                window.height() / 2.,
                1.,
            ),
            EnemySpawnPos::Left => Vec3::new(
                -window.width() / 2.,
                rng.gen_range((-window.height() / 2.0)..(window.height() / 2.0)),
                1.,
            ),
            EnemySpawnPos::Right => Vec3::new(
                window.width() / 2.,
                rng.gen_range((-window.height() / 2.0)..(window.height() / 2.0)),
                1.,
            ),
        };

        commands
            .spawn_bundle(SpriteBundle {
                texture: sprites.enemy.clone(),
                transform: Transform::from_translation(translation).with_scale(Vec3::splat(6.0)),
                ..default()
            })
            .insert(Enemy)
            .insert(AttackTimer(Timer::from_seconds(0.5, true)))
            .insert(Damage(10.0 * enemy_scale.0))
            .insert(Health::new(100.0 * enemy_scale.0))
            .insert(RigidBody::Dynamic)
            .insert(ExternalImpulse::default())
            .insert(ExternalForce::default())
            .insert(ColliderMassProperties::Density(0.0))
            .insert(AdditionalMassProperties::Mass(10.0))
            .insert(Velocity::default())
            .insert(Collider::cuboid(5.0, 7.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

fn move_to_player(
    mut enemies: Query<(&mut Transform, &mut ExternalForce), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player.single();

    for (mut transform, mut velocity) in &mut enemies {
        let dir = (player_transform.translation.truncate() - transform.translation.truncate())
            .normalize();

        transform.rotation = Quat::from_rotation_z(Vec2::X.angle_between(dir));

        velocity.force = dir * 1000.0;
    }
}

fn damage_player(
    time: Res<Time>,
    mut enemies: Query<(&Transform, &mut AttackTimer, &Damage), With<Enemy>>,
    mut player: Query<(&Transform, &mut Health), With<Player>>,
    audio: Res<Audio>,
    sound: Res<Sounds>,
) {
    let (player_transform, mut health) = player.single_mut();

    for (enemy_transform, mut attack_timer, damage) in &mut enemies {
        let dist = player_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());

        const THRESHOLD: f32 = 100.0;
        if dist <= THRESHOLD {
            attack_timer.tick(time.delta());
            if attack_timer.just_finished() {
                audio.play_with_settings(
                    sound.player_hit.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
                health.0 -= damage.0;
            }
        }
    }
}
