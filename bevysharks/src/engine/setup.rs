use bevy::prelude::*;
use crate::engine::base_entities::Boat;

use super::base_entities::{Fish, Shark, HungerCircle};

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

pub fn game_cleanup(mut commands: Commands, 
    mut fish_query: Query<(Entity, With<Fish>)>,
    mut shark_query: Query<(Entity, With<Shark>)>,
    mut player_query: Query<(Entity, With<Boat>)>,
mut hunger_query: Query<(Entity, With<HungerCircle>)>) {
    fish_query.for_each_mut(|(entity, _)| commands.entity(entity).despawn());
    shark_query.for_each_mut(|(entity, _)| commands.entity(entity).despawn());
    player_query.for_each_mut(|(entity, _)| commands.entity(entity).despawn());
    hunger_query.for_each_mut(|(entity, _)| commands.entity(entity).despawn());
}

pub fn game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("game_over_v1.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        });
}

pub fn idle() {}