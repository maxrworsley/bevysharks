use bevy::prelude::*;

fn main() {
    App::new()
    .add_systems(Startup, setup)
    .add_systems(Update, move_player)
    .add_systems(Update, move_sharks)
    .add_systems(Update, spawn_fish)
    .add_systems(Update, check_game_over)
    .run();
}


fn setup() {
    todo!();
}

fn move_player() {
    todo!();
}

fn move_sharks() {
    todo!();
}

fn spawn_fish() {
    todo!();
}

fn check_game_over() {
    todo!();
}

#[derive(Component)]
struct Position(u32, u32);

#[derive(Component)]
struct Velocity(u32, u32);

#[derive(Component)]
struct State {position: Position, velocity: Velocity }

struct Boat{state: State}
struct Shark{state: State}
struct Fish{state: State}