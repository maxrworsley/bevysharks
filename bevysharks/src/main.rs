use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

const MAX_SHARK_VELOCITY: f64 = 90.;
const MAX_PLAYER_VELOCITY: f64 = 60.;
const MAX_PLAYER_ACCELERATION: f64 = 3.;
const PLAYER_CHANGE_VELOCITY: f64 = 10.;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, move_player)
    .add_systems(Update, move_sharks)
    .add_systems(Update, spawn_fish)
    .add_systems(Update, spawn_sharks)
    .add_systems(Update, update_hunger)
    .run();
}


fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    
    // Spawn player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Boat {state: State {
            position: Position(0., 0.), 
            velocity: Velocity(0., 0.)},
            acceleration: Acceleration(0., 0.),
            hunger: Hunger(30.),
        },
    ));


    // Spawn one shark
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(10., 40.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
            ..default()
        },
        Shark {state: State {position: Position(-300., -300.), velocity: Velocity(10., 10.)}},
    ));
}

fn move_player(time: Res<Time>, mut commands: Commands, input: Res<Input<KeyCode>>, 
    mut player_query: Query<(&mut Boat, &mut Transform)>, 
    mut fish_query: Query<(Entity, &Fish)>) {
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
    player.acceleration.0 += acceleration.x as f64;
    player.acceleration.1 += acceleration.y as f64;
    if player.acceleration.0 > MAX_PLAYER_ACCELERATION {player.acceleration.0 = MAX_PLAYER_ACCELERATION;}
    if player.acceleration.0 < -MAX_PLAYER_ACCELERATION {player.acceleration.0 = -MAX_PLAYER_ACCELERATION;}
    if player.acceleration.1 > MAX_PLAYER_ACCELERATION {player.acceleration.1 = MAX_PLAYER_ACCELERATION;}
    if player.acceleration.1 < -MAX_PLAYER_ACCELERATION {player.acceleration.1 = -MAX_PLAYER_ACCELERATION;}

    // Update velocity
    player.state.velocity.0 += acceleration.x as f64;
    player.state.velocity.1 += acceleration.y as f64;

    // Update position
    player.state.position.0 += player.state.velocity.0 * time_delta;
    player.state.position.1 += player.state.velocity.1 * time_delta;

    if player.state.velocity.0 > MAX_PLAYER_VELOCITY {player.state.velocity.0 = MAX_PLAYER_VELOCITY;}
    if player.state.velocity.0 < -MAX_PLAYER_VELOCITY {player.state.velocity.0 = -MAX_PLAYER_VELOCITY;}
    if player.state.velocity.1 > MAX_PLAYER_VELOCITY {player.state.velocity.1 = MAX_PLAYER_VELOCITY;}
    if player.state.velocity.1 < -MAX_PLAYER_VELOCITY {player.state.velocity.1 = -MAX_PLAYER_VELOCITY;}

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

fn move_sharks(time: Res<Time>, mut shark_query: Query<(&mut Shark, &mut Transform)>, player_query: Query<&Boat>, game_over_query: Query<&GameOver>) {
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
        shark.state.velocity.0 += -(angle.cos() * MAX_SHARK_VELOCITY) as f64;
        shark.state.velocity.1 += -(angle.sin() * MAX_SHARK_VELOCITY) as f64;
        // cap velocity
        let per_shark_velocity_cap = MAX_SHARK_VELOCITY * ( rng.gen_range(1..10) as f64 / 10.) as f64;
        if shark.state.velocity.0 > per_shark_velocity_cap {shark.state.velocity.0 = per_shark_velocity_cap;}
        if shark.state.velocity.0 < -per_shark_velocity_cap {shark.state.velocity.0 = -per_shark_velocity_cap;}
        if shark.state.velocity.1 > per_shark_velocity_cap {shark.state.velocity.1 = per_shark_velocity_cap;}
        if shark.state.velocity.1 < -per_shark_velocity_cap {shark.state.velocity.1 = -per_shark_velocity_cap;}
        transform.rotation = Quat::from_rotation_z(angle as f32);

        // If shark is touching player, kill player
        if shark.state.position.0 - player.state.position.0 < 10. &&
        shark.state.position.0 - player.state.position.0 > -10. &&
        shark.state.position.1 - player.state.position.1 < 10. &&
        shark.state.position.1 - player.state.position.1 > -10. {
            println!("You died!");
        }
    }
}

fn spawn_fish(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, 
    fish_query: Query<&Fish>, window: Query<&Window>) {

    if fish_query.iter().count() < 10 {
        let window_width_half = window.single().width() / 2.;
        let window_height_half = window.single().height() / 2.;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-window_width_half..window_width_half);
        let y = rng.gen_range(-window_height_half..window_height_half);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(10., 10.)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            },
            Fish {state: State {position: Position(x as f64, y as f64), velocity: Velocity(0., 0.)}},
        ));
    }
}

fn spawn_sharks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, 
    fish_query: Query<&Fish>, window: Query<&Window>) {

    if fish_query.iter().count() < 10 {
        let window_width_half = window.single().width() / 2.;
        let window_height_half = window.single().height() / 2.;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-window_width_half..window_width_half);
        let y = rng.gen_range(-window_height_half..window_height_half);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(10., 40.)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
                ..default()
            },
            Shark {state: State {position: Position(x as f64, y as f64), velocity: Velocity(10., 10.)}},
        ));
    }
}

fn update_hunger(time: Res<Time>, mut boat_query: Query<&mut Boat>) {
    let time_delta = time.delta_seconds_f64();
    boat_query.single_mut().hunger.0 -= time_delta;

    let hunger = &boat_query.single_mut().hunger;
    if hunger.0 <= 0. {
        boat_query.single_mut().hunger.0 = 0.;
        println!("You died of hunger!");
    }
}

// Components
#[derive(Component)]
struct Position(f64, f64);

#[derive(Component)]
struct Velocity(f64, f64);

#[derive(Component)]
struct State {position: Position, velocity: Velocity }

#[derive(Component)]
struct Acceleration(f64, f64);

#[derive(Component)]
struct Hunger(f64);

// Entities
#[derive(Component)]
struct Boat{state: State, acceleration: Acceleration, hunger: Hunger}

#[derive(Component)]
struct Shark{state: State}

#[derive(Component)]
struct Fish{state: State}

#[derive(Component)]
struct GameOver;