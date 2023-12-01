use bevy::prelude::Component;
use crate::base_components::{State, Position};

#[derive(Component)]
pub struct Boat {
    pub state: State, 
    pub hunger: f64
}

impl Boat {
    pub fn new() -> Boat {
        Boat {
            state: State::new(),
            hunger: 10.
        }
    }
}

#[derive(Component)]
pub struct Shark {
    pub state: State
}

impl Shark {
    pub fn from_position(position: Position) -> Shark {
        Shark {
            state: State::from_position(position)
        }
    }
}

#[derive(Component)]
pub struct Fish {
    pub state: State
}

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct HungerCircle;