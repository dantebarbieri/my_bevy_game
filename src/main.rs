use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};

mod debug;
mod player;
// mod sprites;
mod tilemap;

use debug::DebugPlugin;
use player::PlayerPlugin;
// use sprites::SpritePlugin;
use tilemap::TileMapPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.2;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        // .add_plugin(SpritePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(DebugPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            left: -1. * RESOLUTION,
            right: 1. * RESOLUTION,
            bottom: -1.,
            top: 1.,
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    });
}
