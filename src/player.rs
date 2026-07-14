use bevy::prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, update_player_position);
        app.insert_resource(MovementTimer(Timer::new(
            Duration::from_millis(150),
            TimerMode::Repeating,
        )));
        app.insert_resource(Directions::Left);
    }
}

#[derive(Component)]
struct SnakeHead;

#[derive(Resource)]
struct MovementTimer(Timer);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Transform::default(),
        Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(1.0, 1.0, 1.0)))),
        SnakeHead,
    ));
}

#[derive(Resource)]
enum Directions {
    Left,
    Right,
    Up,
    Down,
}

impl Directions {
    fn update_direction(&mut self, button_input: Res<ButtonInput<KeyCode>>) {
        if button_input.pressed(KeyCode::KeyA) {
            *self = Directions::Left;
        }
        if button_input.pressed(KeyCode::KeyD) {
            *self = Directions::Right;
        }
        if button_input.pressed(KeyCode::KeyW) {
            *self = Directions::Up;
        }
        if button_input.pressed(KeyCode::KeyS) {
            *self = Directions::Down;
        }
    }
}

fn update_player_position(
    button_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<SnakeHead>>,
    mut timer: ResMut<MovementTimer>,
    time: Res<Time>,
    mut direction: ResMut<Directions>,
) {
    let mut transform = query
        .single_mut()
        .expect("There should only be one snake head");
    let delta = 20.0;

    direction.update_direction(button_input);
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        match *direction {
            Directions::Left => transform.translation.x -= delta,
            Directions::Right => transform.translation.x += delta,
            Directions::Down => transform.translation.y -= delta,
            Directions::Up => transform.translation.y += delta,
        }
    }
}
