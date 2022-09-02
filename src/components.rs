use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Barrel;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct FireRate(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct LastShotTime(pub Stopwatch);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
