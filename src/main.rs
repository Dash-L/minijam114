use bevy::{app::AppExit, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
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
}

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct MainMenu;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Minijam 114".to_string(),
            resizable: false,
            ..default()
        })
        .add_loopless_state(GameState::Loading)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .with_collection::<Fonts>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // Our plugins
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_enter_system(GameState::Menu, setup)
        .add_exit_system(GameState::Menu, despawn_with::<MainMenu>)
        .add_system(update_buttons.run_in_state(GameState::Menu))
        .add_system(play.run_if(button_pressed::<PlayButton>))
        .add_system(exit.run_if(button_pressed::<ExitButton>))
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

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            color: UiColor([0.0; 4].into()),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
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
