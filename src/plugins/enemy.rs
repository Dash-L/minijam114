use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use rand::{distributions::Standard, prelude::*};

use crate::{
    components::{Enemy, Player},
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
) {
    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() {
        let window = windows.primary();

        let mut rng = rand::thread_rng();
        let enemy_pos: EnemySpawnPos = rng.gen();

        let translation = match enemy_pos {
            EnemySpawnPos::Up => Vec3::new(0., -window.height() / 2., 1.),
            EnemySpawnPos::Down => Vec3::new(0., window.height() / 2., 1.),
            EnemySpawnPos::Left => Vec3::new(-window.width() / 2., 0., 1.),
            EnemySpawnPos::Right => Vec3::new(window.width() / 2., 0., 1.),
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(16.0, 32.0)),
                    ..default()
                },
                transform: Transform::from_translation(translation),
                ..default()
            })
            .insert(Enemy)
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(Collider::cuboid(8.0, 16.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

fn move_to_player(
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player.single();

    for (transform, mut velocity) in &mut enemies {
        let dir = (player_transform.translation.truncate() - transform.translation.truncate())
            .normalize();

        velocity.linvel = dir * 500.0;
    }
}
