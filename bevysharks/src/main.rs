use bevy::prelude::*;

mod engine;
use engine::base_components;
use engine::sharks::{move_sharks, spawn_sharks};
use engine::player::{move_player, update_hunger};
use engine::fish::spawn_fish;
use engine::setup::{setup, game_cleanup, game_over_screen, idle};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<base_components::GameState>()
    .add_systems(OnEnter(base_components::GameState::InGame), setup)
    .add_systems(
        Update,
        (
            move_player,
            move_sharks,
            spawn_fish,
            spawn_sharks,
            update_hunger).run_if(in_state(base_components::GameState::InGame)),
    )
    .add_systems(OnExit(base_components::GameState::InGame), game_cleanup)
    .add_systems(OnEnter(base_components::GameState::GameOver), game_over_screen)
    .add_systems(Update, idle.run_if(in_state(base_components::GameState::GameOver)))
    .run();
}