use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::Player,
    despawn_with,
    resources::{Fonts, Sprites},
    GameState,
};

#[derive(Component)]
struct PrevVelocity(Velocity);

#[derive(Component)]
struct PrevForce(ExternalForce);

#[derive(Component)]
struct SkillTreeMenu;

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(open_skill_tree.run_in_state(GameState::Playing))
            .add_enter_system(GameState::SkillTree, spawn_skill_tree)
            .add_enter_system(GameState::SkillTree, pause)
            .add_exit_system(GameState::SkillTree, unpause)
            .add_exit_system(GameState::SkillTree, despawn_with::<SkillTreeMenu>)
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

fn spawn_skill_tree(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(NodeBundle {
            color: UiColor([0.0; 4].into()),
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for images in [
                sprites.bullet_type.clone(),
                sprites.spread.clone(),
                sprites.effects.clone(),
            ] {
                parent
                    .spawn_bundle(NodeBundle {
                        color: UiColor([0.0; 4].into()),
                        style: Style {
                            margin: UiRect::all(Val::Auto),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for image in images {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                        margin: UiRect::all(Val::Px(60.0)),
                                        ..default()
                                    },
                                    color: UiColor([0.8; 4].into()),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(ImageBundle {
                                        image: UiImage(image),
                                        ..default()
                                    });
                                    parent.spawn_bundle(ImageBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                                            position_type: PositionType::Absolute,
                                            ..default()
                                        },
                                        image: UiImage(sprites.locks[1].clone()),
                                        ..default()
                                    });
                                });
                        }
                    });
            }
        })
        .insert(SkillTreeMenu);
}

fn pause(
    mut commands: Commands,
    mut entities: Query<
        (
            Entity,
            &mut RigidBody,
            Option<&Velocity>,
            Option<&mut ExternalForce>,
        ),
        Without<Player>,
    >,
) {
    for (entity, mut rigid_body, velocity, force) in &mut entities {
        *rigid_body = RigidBody::Fixed;
        if let Some(velocity) = velocity {
            commands
                .entity(entity)
                .insert(PrevVelocity(velocity.clone()));
        }

        if let Some(mut force) = force {
            commands.entity(entity).insert(PrevForce(force.clone()));
            force.force = Vec2::ZERO;
        }
    }
}

fn unpause(
    mut commands: Commands,
    mut entities: Query<
        (
            Entity,
            &mut RigidBody,
            Option<&mut ExternalForce>,
            Option<&PrevForce>,
            Option<&mut Velocity>,
            Option<&PrevVelocity>,
        ),
        Without<Player>,
    >,
) {
    for (entity, mut rigid_body, force, prev_force, velocity, prev_velocity) in &mut entities {
        *rigid_body = RigidBody::Dynamic;

        if let Some(mut force) = force {
            *force = prev_force.unwrap().0.clone();
        }

        if let Some(mut velocity) = velocity {
            *velocity = prev_velocity.unwrap().0.clone();
        }

        commands
            .entity(entity)
            .remove::<PrevForce>()
            .remove::<PrevVelocity>();
    }
}
