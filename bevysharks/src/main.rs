use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
        Boat {state: State {position: Position(0, 0), velocity: Velocity(0, 0)}},
    ));


    // Spawn one shark
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(10., 40.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
            ..default()
        },
        Shark {state: State {position: Position(0, 0), velocity: Velocity(0, 0)}},
    ));
}

fn move_player() {
    {}
}

fn move_sharks() {
    {}
}

fn spawn_fish() {
    {}
}

fn check_if_game_over() {
    {}
}

#[derive(Component)]
struct Position(u32, u32);

#[derive(Component)]
struct Velocity(u32, u32);

#[derive(Component)]
struct State {position: Position, velocity: Velocity }

#[derive(Component)]
struct Boat{state: State}
#[derive(Component)]
struct Shark{state: State}
#[derive(Component)]
struct Fish{state: State}