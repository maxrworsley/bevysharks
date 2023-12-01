use bevy::prelude::Component;

pub const MAX_SHARK_VELOCITY: f64 = 90.;
pub const MAX_PLAYER_VELOCITY: f64 = 60.;
pub const MAX_PLAYER_ACCELERATION: f64 = 5.;
pub const MAX_FISH_COUNT: usize = 10;

// Components
#[derive(Component)]
pub struct Position(pub f64, pub f64);

#[derive(Component)]
pub struct Velocity(pub f64, pub f64);

#[derive(Component)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Component)]
pub struct State {
    pub position: Position, 
    pub velocity: Velocity,
    pub acceleration: Acceleration
}

impl State {
    pub fn new() -> State {
        State {
            position: Position(0., 0.), 
            velocity: Velocity(0., 0.),
            acceleration: Acceleration(0., 0.)
        }
    }

    pub fn from_position(position: Position) -> State {
        State {
            position: position, 
            velocity: Velocity(0., 0.),
            acceleration: Acceleration(0., 0.)
        }
    }

    pub fn from_position_velocity(position: Position, velocity: Velocity) -> State {
        State {
            position: position, 
            velocity: velocity,
            acceleration: Acceleration(0., 0.)
        }
    }
}

#[derive(Component)]
pub struct Hunger(pub f64);

