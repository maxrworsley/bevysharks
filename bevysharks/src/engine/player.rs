use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::engine::{base_entities, base_components};

pub fn move_player(time: Res<Time>, mut commands: Commands, input: Res<Input<KeyCode>>, 
    mut player_query: Query<(&mut base_entities::Boat, &mut Transform)>, 
    mut fish_query: Query<(Entity, &base_entities::Fish)>) {
    let time_delta = time.delta_seconds_f64();
    let (mut player, mut transform) = player_query.single_mut();

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
    player.state.apply_velocity_to_position(time_delta);
    
    transform.translation.x = player.state.position.0 as f32;
    transform.translation.y = player.state.position.1 as f32;

    // Remove fish if player collides with them
    for (fish_entity, fish) in fish_query.iter_mut() {
        if fish.state.position.0 - player.state.position.0 < 10. && 
        fish.state.position.0 - player.state.position.0 > -10. && 
        fish.state.position.1 - player.state.position.1 < 10. && 
        fish.state.position.1 - player.state.position.1 > -10. {
            commands.entity(fish_entity).despawn(); 
            player.hunger += 8.;
            break;
        }

    }
}

pub fn update_hunger(time: Res<Time>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, 
    mut boat_query: Query<&mut base_entities::Boat>, mut hunger_query: Query<(Entity, With<base_entities::HungerCircle>)>) {

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

fn spawn_hunger_circle(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>, boat: &base_entities::Boat, hunger: &f64) {
    commands.spawn((MaterialMesh2dBundle{
        mesh: meshes.add(shape::Circle::new(*hunger as f32).into()).into(),
        material: materials.add(Color::rgba(0.3, 0.3, 0.5, 0.5).into()),
        transform: Transform::from_translation(
            Vec3::new(boat.state.position.0 as f32, boat.state.position.1 as f32, 0.)
        ),
        ..default()},
        base_entities::HungerCircle));
}
