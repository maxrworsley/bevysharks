use bevy::prelude::*;
use crate::engine::base_entities::{Fish, Shark, Boat};
use crate::engine::base_components::{Position, MAX_FISH_COUNT, MAX_SHARK_VELOCITY};
use crate::engine::geometry_functions::objects_are_touching;
use crate::engine::geometry_functions::get_distance;


pub fn spawn_sharks(mut commands: Commands, asset_server: Res<AssetServer>, fish_query: Query<&Fish>, window: Query<&Window>) {
    if fish_query.iter().count() < MAX_FISH_COUNT {
        let window_width_half = window.single().width() / 2.;
        let window_height_half = window.single().height() / 2.;

        let position = Position::new_in_bounds(window_width_half as f64, window_height_half as f64);
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("shark_v1.png"),
                transform: Transform::from_translation(Vec3::new(position.0 as f32, position.1 as f32, 0.)),
                ..default()
            },
            Shark::from_position(position)
        ));
    }
}

pub fn move_sharks(time: Res<Time>, mut shark_query: Query<(&mut Shark, &mut Transform)>, player_query: Query<&Boat>) {
    let player = player_query.single();
    let time_delta = time.delta_seconds_f64();
    let mut rng = rand::thread_rng();
    let shark_count = shark_query.iter().count();

    // Move sharks to face the player and move forwards
    for (mut shark, mut transform) in shark_query.iter_mut() {
        if get_distance(&shark.state.position, &player.state.position) > 300. {
            shark.state.velocity.0 = 0.;
            shark.state.velocity.1 = 0.;
            continue;
        }
        shark.state.position.0 += shark.state.velocity.0 * time_delta;
        shark.state.position.1 += shark.state.velocity.1 * time_delta;

        transform.translation.x = shark.state.position.0 as f32;
        transform.translation.y = shark.state.position.1 as f32;
        // Angle between shark and player
        let angle = (player.state.position.1 - shark.state.position.1).atan2(player.state.position.0 - shark.state.position.0) + std::f64::consts::PI;
        shark.state.velocity.0 += -(angle.cos() * MAX_SHARK_VELOCITY) as f64;
        shark.state.velocity.1 += -(angle.sin() * MAX_SHARK_VELOCITY) as f64;
        // cap velocity
        let per_shark_velocity_cap = 0.05 * MAX_SHARK_VELOCITY * shark_count as f64;
        if shark.state.velocity.0 > per_shark_velocity_cap {shark.state.velocity.0 = per_shark_velocity_cap;}
        if shark.state.velocity.0 < -per_shark_velocity_cap {shark.state.velocity.0 = -per_shark_velocity_cap;}
        if shark.state.velocity.1 > per_shark_velocity_cap {shark.state.velocity.1 = per_shark_velocity_cap;}
        if shark.state.velocity.1 < -per_shark_velocity_cap {shark.state.velocity.1 = -per_shark_velocity_cap;}
        transform.rotation = Quat::from_rotation_z(angle as f32);

        // If shark is touching player, kill player
        if objects_are_touching(&shark.state.position, 1., &player.state.position, 5.) {
            println!("You died!");
        }
    }
}