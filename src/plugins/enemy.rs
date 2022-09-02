use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(ConditionSet::new().run_in_state(GameState::Playing).into());
    }
}
