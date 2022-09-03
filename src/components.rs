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

// this is horribly misnamed but it's basically a request for an animation to stop :)
#[derive(Component)]
pub struct Ready(pub bool);

#[derive(Component)]
pub struct Health(pub f32, pub f32);

impl Health {
    pub fn new(health: f32) -> Self {
        Self(health, health)
    }
}

#[derive(Component)]
pub struct HasHealthBar;

#[derive(Component)]
pub struct HealthBar(pub bool);

