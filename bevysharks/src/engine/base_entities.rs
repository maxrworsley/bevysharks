use bevy::prelude::Component;
use crate::base_components::{State, Hunger};

#[derive(Component)]
pub struct Boat {
    pub state: State, 
    pub hunger: Hunger
}

#[derive(Component)]
pub struct Shark {
    pub state: State
}

#[derive(Component)]
pub struct Fish {
    pub state: State
}

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct HungerCircle;