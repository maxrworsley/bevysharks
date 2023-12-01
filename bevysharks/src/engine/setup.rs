use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::engine::{base_entities, base_components};

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    
    // Spawn player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        },
        base_entities::Boat {
            state: base_components::State::new(),
            hunger: base_components::Hunger(10.),
        },
    ));
}