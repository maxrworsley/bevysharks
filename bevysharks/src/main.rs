use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const MAX_SHARK_VELOCITY: f64 = 50.;
const MAX_PLAYER_VELOCITY: f64 = 50.;
const PLAYER_CHANGE_VELOCITY: f64 = 10.;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, move_player)
    .add_systems(Update, move_sharks)
    .add_systems(Update, spawn_fish)
    .add_systems(Update, check_if_game_over)
    .run();
}


fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    
    // Spawn player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(3.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Boat {state: State {position: Position(0., 0.), velocity: Velocity(0., 0.)}},
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

fn move_player(time: Res<Time>, input: Res<Input<KeyCode>>, mut player_query: Query<(&mut Boat, &mut Transform)>) {
    let time_delta = time.delta_seconds_f64();
    let (mut player, mut transform) = player_query.single_mut();

    let mut velocity = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        velocity.y += 1.0;
    }
    if input.pressed(KeyCode::S) {
        velocity.y -= 1.0;
    }
    if input.pressed(KeyCode::A) {
        velocity.x -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        velocity.x += 1.0;
    }

    velocity.x *= MAX_PLAYER_VELOCITY as f32;
    velocity.y *= MAX_PLAYER_VELOCITY as f32;
    player.state.velocity.add_velocity(velocity);
    player.state.position.0 += player.state.velocity.0 * time_delta;
    player.state.position.1 += player.state.velocity.1 * time_delta;

    if player.state.velocity.0 > MAX_PLAYER_VELOCITY {player.state.velocity.0 = MAX_PLAYER_VELOCITY;}
    if player.state.velocity.0 < -MAX_PLAYER_VELOCITY {player.state.velocity.0 = -MAX_PLAYER_VELOCITY;}
    if player.state.velocity.1 > MAX_PLAYER_VELOCITY {player.state.velocity.1 = MAX_PLAYER_VELOCITY;}
    if player.state.velocity.1 < -MAX_PLAYER_VELOCITY {player.state.velocity.1 = -MAX_PLAYER_VELOCITY;}

    transform.translation.x = player.state.position.0 as f32;
    transform.translation.y = player.state.position.1 as f32;
}



fn move_sharks(time: Res<Time>, mut shark_query: Query<(&mut Shark, &mut Transform)>, player_query: Query<&Boat>) {
    let player = player_query.single();
    let time_delta = time.delta_seconds_f64();

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
        if shark.state.velocity.0 > MAX_SHARK_VELOCITY {shark.state.velocity.0 = MAX_SHARK_VELOCITY;}
        if shark.state.velocity.0 < -MAX_SHARK_VELOCITY {shark.state.velocity.0 = -MAX_SHARK_VELOCITY;}
        if shark.state.velocity.1 > MAX_SHARK_VELOCITY {shark.state.velocity.1 = MAX_SHARK_VELOCITY;}
        if shark.state.velocity.1 < -MAX_SHARK_VELOCITY {shark.state.velocity.1 = -MAX_SHARK_VELOCITY;}
        transform.rotation = Quat::from_rotation_z(angle as f32);

    }
}

fn spawn_fish() {
    {}
}

fn check_if_game_over() {
    {}
}

// Components
#[derive(Component)]
struct Position(f64, f64);

#[derive(Component)]
struct Velocity(f64, f64);

impl Velocity {
    fn add_velocity(&mut self, velocity: Vec2) {
        self.0 += velocity.x as f64;
        self.1 += velocity.y as f64;
    }
}

#[derive(Component)]
struct State {position: Position, velocity: Velocity }

// Entities
#[derive(Component)]
struct Boat{state: State}
#[derive(Component)]
struct Shark{state: State}
#[derive(Component)]
struct Fish{state: State}