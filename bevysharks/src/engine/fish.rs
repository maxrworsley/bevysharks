use crate::engine::base_entities::Fish;
use crate::engine::base_components::{Position, State, MAX_FISH_COUNT};
use bevy::prelude::*;

pub fn spawn_fish(mut commands: Commands, asset_server: Res<AssetServer>, fish_query: Query<&Fish>, window: Query<&Window>) {

    if fish_query.iter().count() < MAX_FISH_COUNT {
        let window_width_half = window.single().width() / 2.;
        let window_height_half = window.single().height() / 2.;

        let position = Position::new_in_bounds(window_width_half as f64, window_height_half as f64);
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("jellyfish_v1.png"),
                transform: Transform::from_translation(Vec3::new(position.0 as f32, position.1 as f32, 1.)),
                ..default()
            },
            Fish { state: State::from_position(position) })
            );
    }
}
