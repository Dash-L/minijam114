use bevy::prelude::*;
use bevy_pkv::PkvStore;
use iyes_loopless::prelude::*;

use crate::{
    button_pressed,
    components::{Bullet, Damage, Enemy, Knockback, Pierce, Player, Coin},
    despawn_with,
    resources::{BulletType, Coins, Fonts, ShootTimer, Spread, SpawnTimer, EnemyScale},
    update_buttons, GameState,
};

#[derive(Component)]
struct GameOverMenu;

#[derive(Component)]
struct BackButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PkvStore::new("DJGames", "TankShmUp"))
            .add_enter_system(GameState::GameOver, despawn_with::<Enemy>)
            .add_enter_system(GameState::GameOver, despawn_with::<Bullet>)
            .add_enter_system(GameState::GameOver, despawn_with::<Player>)
            .add_enter_system(GameState::GameOver, despawn_with::<Node>)
            .add_enter_system(GameState::GameOver, despawn_with::<Coin>)
            .add_enter_system(GameState::GameOver, show_game_over)
            .add_exit_system(GameState::GameOver, despawn_with::<GameOverMenu>)
            .add_enter_system(GameState::Menu, reset_stats)
            .add_system(update_buttons.run_in_state(GameState::GameOver))
            .add_system(
                to_menu
                    .run_in_state(GameState::GameOver)
                    .run_if(button_pressed::<BackButton>),
            );
    }
}

fn show_game_over(
    mut commands: Commands,
    fonts: Res<Fonts>,
    coins: Res<Coins>,
    mut pkv: ResMut<PkvStore>,
) {
    let high_score = if let Ok(high_score) = pkv.get::<u32>("high_score") {
        if coins.0 > high_score {
            pkv.set::<u32>("high_score", &coins.0)
                .expect("failed to access pkv store");

            coins.0
        } else {
            high_score
        }
    } else {
        pkv.set::<u32>("high_score", &coins.0)
            .expect("failed to access pkv store");

        coins.0
    };

    commands
        .spawn_bundle(NodeBundle {
            color: UiColor([0.0; 4].into()),
            style: Style {
                margin: UiRect::all(Val::Auto),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .insert(GameOverMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Game Over!",
                TextStyle {
                    color: Color::RED,
                    font: fonts.main.clone(),
                    font_size: 60.0,
                },
            ));

            parent.spawn_bundle(TextBundle::from_section(
                format!("Score: {}", coins.0),
                TextStyle {
                    color: Color::YELLOW,
                    font: fonts.main.clone(),
                    font_size: 50.0,
                },
            ));

            parent.spawn_bundle(TextBundle::from_section(
                format!("High Score: {}", high_score),
                TextStyle {
                    color: Color::YELLOW,
                    font: fonts.main.clone(),
                    font_size: 50.0,
                },
            ));

            parent
                .spawn_bundle(ButtonBundle {
                    color: UiColor(Color::BLACK),
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(BackButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            color: Color::WHITE,
                            font: fonts.main.clone(),
                            font_size: 35.0,
                        },
                    ));
                });
        });
}

fn to_menu(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Menu));
}

fn reset_stats(
    mut coins: ResMut<Coins>,
    mut damage: ResMut<Damage>,
    mut knockback: ResMut<Knockback>,
    mut spread: ResMut<Spread>,
    mut pierce: ResMut<Pierce>,
    mut shoot_timer: ResMut<ShootTimer>,
    mut bullet_type: ResMut<BulletType>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut enemy_scale: ResMut<EnemyScale>,
) {
    coins.0 = 0;
    *damage = default();
    *knockback = default();
    *spread = default();
    *pierce = default();
    *shoot_timer = default();
    *bullet_type = default();

    *spawn_timer = default();
    *enemy_scale = default();
}
