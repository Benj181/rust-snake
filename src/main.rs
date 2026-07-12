use bevy::prelude::*;
mod player;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
