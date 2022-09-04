use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use rand::{distributions::Standard, prelude::*};

use crate::{
    components::{AttackTimer, Bullet, Damage, Enemy, Health, ImmobileTimer, Player},
    resources::{HasSuck, Sprites},
    GameState,
};

pub struct EnemyPlugin;
pub enum EnemySpawnPos {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Deref, DerefMut)]
struct SpawnTimer(Timer);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(spawn_enemies)
                    .with_system(move_to_player)
                    .with_system(damage_player)
                    .with_system(handle_freeze)
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

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
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
            .insert(Damage(10.0))
            .insert(Health::new(100.0))
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
    has_suck: Res<HasSuck>,
    mut enemies: Query<(&mut Transform, &mut ExternalForce), (With<Enemy>, Without<ImmobileTimer>)>,
    bullets: Query<&Transform, (With<Bullet>, Without<Enemy>)>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player.single();

    for (mut transform, mut velocity) in &mut enemies {
        let dir = (player_transform.translation.truncate() - transform.translation.truncate())
            .normalize();

        transform.rotation = Quat::from_rotation_z(Vec2::X.angle_between(dir));

        let bias = if has_suck.0 {
            let mut res = Vec2::ZERO;

            for bullet_transform in &bullets {
                let dir = bullet_transform.translation.truncate() - transform.translation.truncate();

                res += dir.normalize() * 150000.0 / dir.length();
            }

            res
        } else {
            Vec2::ZERO
        };

        velocity.force = dir * 1000.0 + bias;
    }
}

fn handle_freeze(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<
        (
            Entity,
            &mut ImmobileTimer,
            &mut Sprite,
            &mut ExternalForce,
            &mut Velocity,
        ),
        With<Enemy>,
    >,
) {
    for (entity, mut immobile_timer, mut sprite, mut force, mut velocity) in &mut enemies {
        immobile_timer.tick(time.delta());
        force.force = Vec2::ZERO;
        velocity.linvel = Vec2::ZERO;
        sprite.color = Color::rgb(0.2, 1.0, 0.1);
        if immobile_timer.just_finished() {
            commands.entity(entity).remove::<ImmobileTimer>();
            sprite.color = [1.0; 3].into();
        }
    }
}

fn damage_player(
    time: Res<Time>,
    mut enemies: Query<(&Transform, &mut AttackTimer, &Damage), With<Enemy>>,
    mut player: Query<(&Transform, &mut Health), With<Player>>,
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
                health.0 -= damage.0;
            }
        }
    }
}
