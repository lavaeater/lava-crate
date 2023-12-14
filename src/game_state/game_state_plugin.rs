use bevy::app::{App, Plugin};
use bevy::prelude::{Event, EventReader, NextState, ResMut};
use crate::game_state::clear_game_entities_plugin::ClearGameEntitiesPlugin;
use crate::game_state::GameState;
use crate::game_state::score_keeper::ScoreKeeperPlugin;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_event::<GotoState>()
            .add_plugins((
                ClearGameEntitiesPlugin,
                ScoreKeeperPlugin,
            ))
            ;
    }
}

pub fn goto_state_system(
    mut state: ResMut<NextState<GameState>>,
    mut goto_state_er: EventReader<GotoState>,
) {
    for goto_state in &mut goto_state_er.read() {
        state.set(goto_state.state.clone());
    }
}

#[derive(Event)]
pub struct GotoState {
    pub state: GameState,
}