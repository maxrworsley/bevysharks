use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::engine::base_entities::{Boat, Fish, HungerCircle};
use crate::engine::geometry_functions::objects_are_touching;

pub fn move_player(time: Res<Time>, mut commands: Commands, input: Res<Input<KeyCode>>, 
    mut player_query: Query<(&mut Boat, &mut Transform)>, mut fish_query: Query<(Entity, &Fish)>, window: Query<&Window>) {
    let time_delta = time.delta_seconds_f64();
    let (mut player, mut transform) = player_query.single_mut();
    let window_width_half = window.single().width() / 2.;
    let window_height_half = window.single().height() / 2.;

    let mut acceleration = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        acceleration.y += 10.;
    }
    if input.pressed(KeyCode::S) {
        acceleration.y -= 10.;
    }
    if input.pressed(KeyCode::A) {
        acceleration.x -= 10.;
    }
    if input.pressed(KeyCode::D) {
        acceleration.x += 10.;
    }

    player.state.apply_acceleration(acceleration.x as f64, acceleration.y  as f64, time_delta);
    player.state.apply_acceleration_to_velocity(time_delta);
    player.state.apply_velocity_to_position(time_delta, window_width_half as f64, window_height_half as f64);
    
    transform.translation.x = player.state.position.0 as f32;
    transform.translation.y = player.state.position.1 as f32;

    // Rotate sprite to face direction of movement
    if player.state.velocity.0 != 0. || player.state.velocity.1 != 0. {
        transform.rotation = Quat::from_rotation_z(
            ((player.state.velocity.1).atan2(player.state.velocity.0) + std::f64::consts::PI) as f32
        );
    }

    // Remove fish if player collides with them
    for (fish_entity, fish) in fish_query.iter_mut() {
        if objects_are_touching(&player.state.position, 8., &fish.state.position, 1.) {
            commands.entity(fish_entity).despawn(); 
            player.hunger += 8.;
        }

    }
}

pub fn update_hunger(time: Res<Time>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, 
    mut boat_query: Query<&mut Boat>, mut hunger_query: Query<(Entity, With<HungerCircle>)>) {

    let time_delta = time.delta_seconds_f64();
    let mut boat = boat_query.single_mut();

    hunger_query.for_each_mut(|(entity, _)| commands.entity(entity).despawn());

    boat.hunger -= time_delta;
    spawn_hunger_circle(&mut commands, &mut meshes, &mut materials, &boat, &boat.hunger);

    if boat.hunger <= 0. {
        boat_query.single_mut().hunger = 0.;
        println!("You died of hunger!");
    }
}

fn spawn_hunger_circle(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>, boat: &Boat, hunger: &f64) {
    commands.spawn((MaterialMesh2dBundle{
        mesh: meshes.add(shape::Circle::new(*hunger as f32).into()).into(),
        material: materials.add(Color::rgba(0.3, 0.3, 0.5, 0.5).into()),
        transform: Transform::from_translation(
            Vec3::new(boat.state.position.0 as f32, boat.state.position.1 as f32, 0.)
        ),
        ..default()},
        HungerCircle));
}
