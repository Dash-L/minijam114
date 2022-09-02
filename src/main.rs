use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    Loading,
    Menu,
    Playing,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Minijam 114".to_string(),
            resizable: false,
            ..default()
        })
        .add_loopless_state(GameState::Loading)
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
