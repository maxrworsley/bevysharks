use crate::engine::{base_entities, base_components};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_fish(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, 
    fish_query: Query<&base_entities::Fish>, window: Query<&Window>) {

    if fish_query.iter().count() < base_components::MAX_FISH_COUNT {
        let window_width_half = window.single().width() / 2.;
        let window_height_half = window.single().height() / 2.;

        let position = base_components::Position::new_in_bounds(window_width_half as f64, window_height_half as f64);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(10., 10.)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(position.0 as f32, position.1 as f32, 1.)),
                ..default()
            },
            base_entities::Fish { state: base_components::State::from_position(position) })
            );
    }
}
