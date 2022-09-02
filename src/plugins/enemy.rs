use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::{distributions::Standard, prelude::*};

use crate::GameState;

pub struct EnemyPlugin;
pub enum EnemySpawnPos {
    Up,
    Down,
    Left,
    Right,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(spawn_enemies)
                .into(),
        );
    }
}

impl Distribution<EnemySpawnPos> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemySpawnPos {
        let val = rng.gen_range(1..5);
        match val {
            1 => EnemySpawnPos::Up,
            2 => EnemySpawnPos::Down,
            3 => EnemySpawnPos::Left,
            4 => EnemySpawnPos::Right,
            _ => unreachable!(),
        }
    }
}

fn spawn_enemies(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.primary();
    let mut rng = rand::thread_rng();
    let enemy_pos: EnemySpawnPos = rng.gen();
    let translation = match enemy_pos {
        EnemySpawnPos::Up => Vec3::new(0., -window.height() / 2., 1.),
        EnemySpawnPos::Down => Vec3::new(0., window.height() / 2., 1.),
        EnemySpawnPos::Left => Vec3::new(-window.width() / 2., 0., 1.),
        EnemySpawnPos::Right => Vec3::new(window.width() / 2., 0., 1.),
    };
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        transform: Transform::from_translation(translation),
        ..default()
    });
}
