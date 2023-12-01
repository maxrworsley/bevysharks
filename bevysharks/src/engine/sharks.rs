use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::engine::{base_entities, base_components};
use rand::Rng;


pub fn spawn_sharks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, 
    fish_query: Query<&base_entities::Fish>, window: Query<&Window>) {

    if fish_query.iter().count() < base_components::MAX_FISH_COUNT {
        let window_width_half = window.single().width() / 2.;
        let window_height_half = window.single().height() / 2.;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-window_width_half..window_width_half);
        let y = rng.gen_range(-window_height_half..window_height_half);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(10., 40.)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            base_entities::Shark {
                state: base_components::State::from_position(base_components::Position(x as f64, y as f64)) }
        ));
    }
}

pub fn move_sharks(time: Res<Time>, mut shark_query: Query<(&mut base_entities::Shark, &mut Transform)>, player_query: Query<&base_entities::Boat>) {
    let player = player_query.single();
    let time_delta = time.delta_seconds_f64();
    let mut rng = rand::thread_rng();

    // Move sharks to face the player and move forwards
    for (mut shark, mut transform) in shark_query.iter_mut() {
        shark.state.position.0 += shark.state.velocity.0 * time_delta;
        shark.state.position.1 += shark.state.velocity.1 * time_delta;

        transform.translation.x = shark.state.position.0 as f32;
        transform.translation.y = shark.state.position.1 as f32;
        // Angle between shark and player
        let angle = (player.state.position.1 - shark.state.position.1).atan2(player.state.position.0 - shark.state.position.0) + std::f64::consts::PI;
        shark.state.velocity.0 += -(angle.cos() * base_components::MAX_SHARK_VELOCITY) as f64;
        shark.state.velocity.1 += -(angle.sin() * base_components::MAX_SHARK_VELOCITY) as f64;
        // cap velocity
        let per_shark_velocity_cap = base_components::MAX_SHARK_VELOCITY * ( rng.gen_range(1..10) as f64 / 10.) as f64;
        if shark.state.velocity.0 > per_shark_velocity_cap {shark.state.velocity.0 = per_shark_velocity_cap;}
        if shark.state.velocity.0 < -per_shark_velocity_cap {shark.state.velocity.0 = -per_shark_velocity_cap;}
        if shark.state.velocity.1 > per_shark_velocity_cap {shark.state.velocity.1 = per_shark_velocity_cap;}
        if shark.state.velocity.1 < -per_shark_velocity_cap {shark.state.velocity.1 = -per_shark_velocity_cap;}
        transform.rotation = Quat::from_rotation_z(angle as f32 - (std::f32::consts::PI / 2.));

        // If shark is touching player, kill player
        if shark.state.position.0 - player.state.position.0 < 10. &&
        shark.state.position.0 - player.state.position.0 > -10. &&
        shark.state.position.1 - player.state.position.1 < 10. &&
        shark.state.position.1 - player.state.position.1 > -10. {
            println!("You died!");
        }
    }
}