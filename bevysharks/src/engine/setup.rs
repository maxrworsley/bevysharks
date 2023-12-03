use bevy::prelude::*;
use crate::engine::base_entities::Boat;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    
    // Spawn player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("boat_v1.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        },
        Boat::new()
    ));
}