use bevy::{prelude::*, utils::HashSet};

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

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);

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

#[derive(Component, Default)]
pub struct HitEnemies(pub HashSet<Entity>);

#[derive(Component, Clone)]
pub struct Pierce(pub i32);

#[derive(Component, Clone)]
pub struct Damage(pub f32);

#[derive(Component, Clone)]
pub struct Knockback(pub f32);

#[derive(Component)]
pub struct HasHealthBar;

#[derive(Component)]
pub struct HealthBar(pub bool);
