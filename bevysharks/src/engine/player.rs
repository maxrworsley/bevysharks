use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::engine::{base_entities, base_components};

pub fn move_player(time: Res<Time>, mut commands: Commands, input: Res<Input<KeyCode>>, 
    mut player_query: Query<(&mut base_entities::Boat, &mut Transform)>, 
    mut fish_query: Query<(Entity, &base_entities::Fish)>) {
    let time_delta = time.delta_seconds_f64();
    let (mut player, mut transform) = player_query.single_mut();

    let mut acceleration = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        acceleration.y += 1.0;
    }
    if input.pressed(KeyCode::S) {
        acceleration.y -= 1.0;
    }
    if input.pressed(KeyCode::A) {
        acceleration.x -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        acceleration.x += 1.0;
    }

    // Update acceleration
    player.state.acceleration.0 += acceleration.x as f64;
    player.state.acceleration.1 += acceleration.y as f64;
    if player.state.acceleration.0 > base_components::MAX_PLAYER_ACCELERATION {player.state.acceleration.0 = base_components::MAX_PLAYER_ACCELERATION;}
    if player.state.acceleration.0 < -base_components::MAX_PLAYER_ACCELERATION {player.state.acceleration.0 = -base_components::MAX_PLAYER_ACCELERATION;}
    if player.state.acceleration.1 > base_components::MAX_PLAYER_ACCELERATION {player.state.acceleration.1 = base_components::MAX_PLAYER_ACCELERATION;}
    if player.state.acceleration.1 < -base_components::MAX_PLAYER_ACCELERATION {player.state.acceleration.1 = -base_components::MAX_PLAYER_ACCELERATION;}

    // Update velocity
    player.state.velocity.0 += acceleration.x as f64;
    player.state.velocity.1 += acceleration.y as f64;

    // Update position
    player.state.position.0 += player.state.velocity.0 * time_delta;
    player.state.position.1 += player.state.velocity.1 * time_delta;

    if player.state.velocity.0 > base_components::MAX_PLAYER_VELOCITY {player.state.velocity.0 = base_components::MAX_PLAYER_VELOCITY;}
    if player.state.velocity.0 < -base_components::MAX_PLAYER_VELOCITY {player.state.velocity.0 = -base_components::MAX_PLAYER_VELOCITY;}
    if player.state.velocity.1 > base_components::MAX_PLAYER_VELOCITY {player.state.velocity.1 = base_components::MAX_PLAYER_VELOCITY;}
    if player.state.velocity.1 < -base_components::MAX_PLAYER_VELOCITY {player.state.velocity.1 = -base_components::MAX_PLAYER_VELOCITY;}

    transform.translation.x = player.state.position.0 as f32;
    transform.translation.y = player.state.position.1 as f32;

    // Remove fish if player collides with them
    for (fish_entity, fish) in fish_query.iter_mut() {
        if fish.state.position.0 - player.state.position.0 < 10. && 
        fish.state.position.0 - player.state.position.0 > -10. && 
        fish.state.position.1 - player.state.position.1 < 10. && 
        fish.state.position.1 - player.state.position.1 > -10. {
            commands.entity(fish_entity).despawn(); 
            player.hunger.0 += 10.;
            break;
        }

    }
}

pub fn update_hunger(time: Res<Time>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, 
    mut boat_query: Query<&mut base_entities::Boat>, mut hunger_query: Query<(Entity, With<base_entities::HungerCircle>)>) {
    // Remove old hunger circles
    for (entity, _) in hunger_query.iter_mut() {
        commands.entity(entity).despawn();
    }
    let time_delta = time.delta_seconds_f64();
    let mut boat = boat_query.single_mut();
    boat.hunger.0 -= time_delta;

    let hunger = &boat.hunger;
    // Render hunger in the form of a circle centered around the player

    let player_x = boat.state.position.0 as f32;
    let player_y = boat.state.position.1 as f32;
    commands.spawn((MaterialMesh2dBundle{
        mesh: meshes.add(shape::Circle::new(hunger.0 as f32).into()).into(),
        material: materials.add(Color::rgba(0.3, 0.3, 0.5, 0.5).into()),
        transform: Transform::from_translation(Vec3::new(player_x, player_y, 0.)),
        ..default()},
        base_entities::HungerCircle{}));

    if hunger.0 <= 0. {
        boat_query.single_mut().hunger.0 = 0.;
        println!("You died of hunger!");
    }
}