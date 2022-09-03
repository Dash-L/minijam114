use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Barrel;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Ready(pub bool);
