use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{components::Player, GameState};

#[derive(Component)]
struct PrevVel(Velocity);

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(open_skill_tree.run_in_state(GameState::Playing))
            .add_enter_system(GameState::SkillTree, pause)
            .add_exit_system(GameState::SkillTree, unpause)
            .add_system(close_skill_tree.run_in_state(GameState::SkillTree));
    }
}

fn open_skill_tree(mut commands: Commands, mouse: Res<Input<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Right) {
        commands.insert_resource(NextState(GameState::SkillTree));
    }
}

fn close_skill_tree(mut commands: Commands, mouse: Res<Input<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Right) {
        commands.insert_resource(NextState(GameState::Playing));
    }
}

fn pause(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut RigidBody, &Velocity), Without<Player>>,
) {
    for (entity, mut rigid_body, velocity) in &mut entities {
        *rigid_body = RigidBody::Fixed;
        commands.entity(entity).insert(PrevVel(velocity.clone()));
    }
}

fn unpause(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut RigidBody, &mut Velocity, &PrevVel), Without<Player>>,
) {
    for (entity, mut rigid_body, mut velocity, prev_vel) in &mut entities {
        *rigid_body = RigidBody::Dynamic;
        *velocity = prev_vel.0.clone();
        commands.entity(entity).remove::<PrevVel>();
    }
}
