use bevy::prelude::*;

mod engine;
use engine::base_components;
use engine::sharks::{move_sharks, spawn_sharks};
use engine::player::{move_player, update_hunger};
use engine::fish::spawn_fish;
use engine::setup::setup;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, move_player)
    .add_systems(Update, move_sharks)
    .add_systems(Update, spawn_fish)
    .add_systems(Update, spawn_sharks)
    .add_systems(Update, update_hunger)
    .run();
}