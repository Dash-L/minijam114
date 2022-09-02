use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(ConditionSet::new().run_in_state(GameState::Playing).into());
    }
}
