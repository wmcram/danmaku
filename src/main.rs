use bevy::prelude::*;

mod bullet;
use bullet::{spawn_bullet, move_bullets, bullet_actions};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_bullets, bullet_actions))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn_bullet(&mut commands, &asset_server);
}


