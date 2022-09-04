use bevy::{app::AppExit, prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use components::{Coin, HasHealthBar, Health, HealthBar, Player};
use iyes_loopless::prelude::*;

mod plugins;
use plugins::*;

mod components;

mod resources;
use resources::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    Loading,
    Menu,
    Playing,
    SkillTree,
}

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct MainMenu;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Minijam 114".to_string(),
            resizable: false,
            ..default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_loopless_state(GameState::Loading)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .with_collection::<Fonts>()
                .with_collection::<Sprites>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // Our plugins
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(SkillTreePlugin)
        .add_enter_system(GameState::Menu, setup)
        .add_exit_system(GameState::Menu, despawn_with::<MainMenu>)
        .add_system(update_buttons.run_in_state(GameState::Menu))
        .add_system(play.run_if(button_pressed::<PlayButton>))
        .add_system(exit.run_if(button_pressed::<ExitButton>))
        // health bar systems (could be a plugin but it's simple enough...)
        .add_system_to_stage(
            CoreStage::PreUpdate,
            remove_at_zero.run_in_state(GameState::Playing),
        )
        .add_system(collide_coins.run_in_state(GameState::Playing))
        .add_system(update_coin_count.run_not_in_state(GameState::Loading))
        .add_system(update_healthbars)
        .add_system(insert_healthbars)
        .run();
}

pub fn despawn_with<C: Component>(mut commands: Commands, q: Query<Entity, With<C>>) {
    for entity in &q {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_pressed<B: Component>(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in &q {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

fn update_buttons(
    mut q: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut q {
        *color = UiColor(match interaction {
            Interaction::Clicked => [0.4; 3].into(),
            Interaction::Hovered => [0.3; 3].into(),
            Interaction::None => [0.0; 3].into(),
        })
    }
}

fn update_healthbars(
    windows: Res<Windows>,
    entities: Query<
        (&Health, &Children, &Transform, Option<&Player>),
        (With<HasHealthBar>, Without<HealthBar>),
    >,
    mut healthbars: Query<(&HealthBar, &mut Transform)>,
) {
    for (health, children, parent_transform, maybe_player) in &entities {
        for child in children {
            if let Ok((healthbar, mut transform)) = healthbars.get_mut(*child) {
                if healthbar.0 {
                    transform.scale.y = (health.0 / health.1) * 80.0 / parent_transform.scale.y;
                }

                if maybe_player.is_some() {
                    let window = windows.primary();
                    transform.translation = Vec3::new(
                        (window.height() - 50.0) / 10.0,
                        (window.width() - 150.0) / 10.0,
                        if healthbar.0 { 11.0 } else { 10.0 },
                    );
                }
            }
        }
    }
}

fn insert_healthbars(
    mut commands: Commands,
    windows: Res<Windows>,
    entities: Query<(Entity, &Transform, Option<&Player>), (With<Health>, Without<HasHealthBar>)>,
) {
    for (entity, transform, maybe_player) in &entities {
        commands
            .entity(entity)
            .with_children(|parent| {
                let healthbar_pos = if maybe_player.is_some() {
                    let window = windows.primary();
                    Vec3::new(
                        (window.height() - 50.0) / 10.0,
                        (window.width() - 150.0) / 10.0,
                        0.0,
                    )
                } else {
                    Vec3::ZERO
                };

                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(8.0, 1.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(healthbar_pos + Vec3::Z * 11.0)
                            .with_scale(Vec3::new(1.0, 80.0, 1.0) / transform.scale),
                        ..default()
                    })
                    .insert(HealthBar(true));

                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(8.0, 1.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(healthbar_pos + Vec3::Z * 10.0)
                            .with_scale(Vec3::new(1.0, 80.0, 1.0) / transform.scale),
                        ..default()
                    })
                    .insert(HealthBar(false));
            })
            .insert(HasHealthBar);
    }
}

fn remove_at_zero(
    mut commands: Commands,
    sprites: Res<Sprites>,
    entities: Query<(Entity, &Transform, &Health), Without<Player>>,
    player: Query<&Transform, With<Player>>,
) {
    for (entity, transform, health) in &entities {
        if health.0 <= 0.0 {
            let player_transform = player.single();

            commands
                .spawn_bundle(SpriteBundle {
                    texture: sprites.coin.clone(),
                    transform: Transform::from_translation(transform.translation)
                        .with_scale(Vec3::splat(4.0)),
                    ..default()
                })
                .insert(Coin)
                .insert(RigidBody::KinematicVelocityBased)
                .insert(Collider::cuboid(4.0, 4.0))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ActiveCollisionTypes::KINEMATIC_STATIC)
                .insert(Velocity::linear(
                    (player_transform.translation.truncate() - transform.translation.truncate())
                        .normalize()
                        * 500.0,
                ));

            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_coin_count(coins: Res<Coins>, mut coins_text: Query<&mut Text, With<Coin>>) {
    if coins.is_changed() {
        if let Ok(mut text) = coins_text.get_single_mut() {
            text.sections[0].value = format!("{}", coins.0);
        }
    }
}

fn collide_coins(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut coins: ResMut<Coins>,
    player: Query<Entity, With<Player>>,
    coins_q: Query<Entity, With<Coin>>,
) {
    for ev in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = ev {
            let (coin_entity, maybe_player) = if let Ok(_) = coins_q.get(*e1) {
                (e1, e2)
            } else if let Ok(_) = coins_q.get(*e2) {
                (e2, e1)
            } else {
                continue;
            };

            if let Ok(_) = player.get(*maybe_player) {
                coins.0 += 1;
                commands.entity(*coin_entity).despawn_recursive();
            }
        }
    }
}

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            color: UiColor([0.0; 4].into()),
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let button_style = Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Px(6.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            };

            let text_style = TextStyle {
                font: fonts.main.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            };

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Play", text_style.clone()));
                })
                .insert(PlayButton);

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Exit", text_style.clone()));
                })
                .insert(ExitButton);
        })
        .insert(MainMenu);
}

fn play(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing));
}

fn exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}
