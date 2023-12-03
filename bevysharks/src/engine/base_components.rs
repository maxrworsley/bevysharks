use bevy::prelude::Component;
use rand::Rng;

pub const MAX_SHARK_VELOCITY: f64 = 90.;
pub const MAX_PLAYER_VELOCITY: f64 = 20.;
pub const MAX_PLAYER_ACCELERATION: f64 = 9.;
pub const MAX_FISH_COUNT: usize = 10;
pub const CHANGE_FACTOR: f64 = 8.;

// Components
#[derive(Component)]
pub struct Position(pub f64, pub f64);

impl Position {
    pub fn new_in_bounds(x_bound: f64, y_bound: f64) -> Position {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-x_bound..x_bound);
        let y = rng.gen_range(-y_bound..y_bound);
        Position(x, y)
    }
}

#[derive(Component)]
pub struct Velocity(pub f64, pub f64);

#[derive(Component)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Component)]
pub struct State {
    pub position: Position, 
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    max_velocity: f64,
    max_acceleration: f64
}

impl State {
    pub fn new() -> State {
        State {
            position: Position(0., 0.), 
            velocity: Velocity(0., 0.),
            acceleration: Acceleration(0., 0.),
            max_velocity: MAX_PLAYER_VELOCITY,
            max_acceleration: MAX_PLAYER_ACCELERATION
        }
    }

    pub fn from_position(position: Position) -> State {
        State {
            position: position, 
            velocity: Velocity(0., 0.),
            acceleration: Acceleration(0., 0.),
            max_velocity: MAX_PLAYER_VELOCITY,
            max_acceleration: MAX_PLAYER_ACCELERATION
        }
    }

    pub fn apply_acceleration(&mut self, change_x: f64, change_y: f64, time_delta: f64) {
        if change_x == 0. {
            if self.acceleration.0 > 0. {
                self.acceleration.0 *= 50. * time_delta * CHANGE_FACTOR;
                if self.acceleration.0 < 0. {self.acceleration.0 = 0.;}
            }
        }
        if change_y == 0. {
            if self.acceleration.1 > 0. {
                self.acceleration.1 *= 50. * time_delta * CHANGE_FACTOR;
            }
        }
        self.acceleration.0 += change_x * time_delta * CHANGE_FACTOR;
        self.acceleration.1 += change_y * time_delta * CHANGE_FACTOR;
        if self.acceleration.0 > self.max_acceleration {self.acceleration.0 = self.max_acceleration;}
        if self.acceleration.0 < -self.max_acceleration {self.acceleration.0 = -self.max_acceleration;}
        if self.acceleration.1 > self.max_acceleration {self.acceleration.1 = self.max_acceleration;}
        if self.acceleration.1 < -self.max_acceleration {self.acceleration.1 = -self.max_acceleration;}
    }

    pub fn apply_acceleration_to_velocity(&mut self, time_delta: f64) {
        self.velocity.0 += self.acceleration.0 * time_delta * CHANGE_FACTOR;
        self.velocity.1 += self.acceleration.1 * time_delta * CHANGE_FACTOR; 
        if self.velocity.0 > self.max_velocity {self.velocity.0 = self.max_velocity;}
        if self.velocity.0 < -self.max_velocity {self.velocity.0 = -self.max_velocity;}
        if self.velocity.1 > self.max_velocity {self.velocity.1 = self.max_velocity;}
        if self.velocity.1 < -self.max_velocity {self.velocity.1 = -self.max_velocity;}
    }

    pub fn apply_velocity_to_position(&mut self, time_delta: f64, x_bound: f64, y_bound: f64) {
        if self.position.0 > x_bound {self.position.0 = -x_bound;}
        if self.position.0 < -x_bound {self.position.0 = x_bound;}
        if self.position.1 > y_bound {self.position.1 = -y_bound;}
        if self.position.1 < -y_bound {self.position.1 = y_bound;}
        self.position.0 += self.velocity.0 * time_delta * CHANGE_FACTOR;
        self.position.1 += self.velocity.1 * time_delta * CHANGE_FACTOR;
    }
}
