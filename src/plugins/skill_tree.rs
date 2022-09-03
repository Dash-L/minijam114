use bevy::{prelude::*, ui::FocusPolicy};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{Damage, Knockback, Pierce, Player},
    despawn_with,
    resources::{BulletType, Fonts, HasIce, HasSuck, Sprites},
    GameState,
};

#[derive(Component)]
struct PrevVelocity(Velocity);

#[derive(Component)]
struct PrevForce(ExternalForce);

#[derive(Component)]
struct Lock(bool);

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
                        for (idx, image) in images.iter().enumerate() {
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
                                        image: UiImage(image.clone()),
                                        focus_policy: FocusPolicy::Pass,
                                        ..default()
                                    });
                                    parent
                                        .spawn_bundle(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                                                position_type: PositionType::Absolute,
                                                ..default()
                                            },
                                            image: UiImage(sprites.locks[1 - idx].clone()),
                                            focus_policy: FocusPolicy::Pass,
                                            ..default()
                                        })
                                        .insert(Lock(idx == 1));
                                });
                        }
                    });
            }
        })
        .insert(SkillTreeMenu);
}

fn handle_button_press(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut pierce: ResMut<Pierce>,
    mut damage: ResMut<Damage>,
    mut knockback: ResMut<Knockback>,
    mut bullet_type: ResMut<BulletType>,
    mut has_ice: ResMut<HasIce>,
    mut has_suck: ResMut<HasSuck>,
    buttons: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    icons: Query<&Handle<Image>, Without<Lock>>,
    mut locks: Query<(Entity, &mut Lock, &mut Handle<Image>)>,
) {
    for (interaction, children) in &buttons {
        if *interaction == Interaction::Clicked {
            let mut icon = None;
            let mut lock = None;

            for child in children {
                if let Ok(actual_icon) = icons.get(*child) {
                    icon = Some(actual_icon);
                } else if let Ok((lock_entity, _, _)) = locks.get(*child) {
                    lock = Some(lock_entity);
                }
            }

            let icon_image = icon.unwrap();
            if let Some(lock) = lock {
                let (lock_entity, mut lock, mut lock_image) = locks.get_mut(lock).unwrap();

                if lock.0 {
                    if icon_image.clone() == sprites.bullet_type[0].clone() {
                    } else if icon_image.clone() == sprites.bullet_type[1].clone() {
                    } else if icon_image.clone() == sprites.spread[0].clone() {
                    } else if icon_image.clone() == sprites.spread[1].clone() {
                    } else if icon_image.clone() == sprites.effects[0].clone() {
                    } else if icon_image.clone() == sprites.effects[1].clone() {
                    }

                    commands.entity(lock_entity).despawn_recursive();
                } else {
                    lock.0 = true;
                    *lock_image = sprites.locks[1].clone();
                }
            }
        }
    }
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
