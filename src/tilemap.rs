use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

#[derive(Component)]
pub struct TileCollider;

fn create_simple_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grass = asset_server.load("textures/rpg/tiles/generic-rpg-tile70.png");
    let dirt = asset_server.load("textures/rpg/tiles/generic-rpg-tile71.png");
    let fence = asset_server.load("textures/rpg/props/generic-rpg-fence03.png");

    let mut tiles = Vec::new();

    let file = File::open("assets/maps/path.bg").expect("No map found");

    let mut bg_line_count = 0_usize;
    let mut bg_char_count = 0_usize;
    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            let mut current_char_count = 0_usize;
            bg_line_count += 1;
            for (x, c) in line.chars().enumerate() {
                current_char_count += 1;
                if c != 'g' && c != 'd' {
                    continue;
                }
                tiles.push(
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(TILE_SIZE)),
                                ..default()
                            },
                            texture: match c {
                                'g' => grass.clone(),
                                'd' => dirt.clone(),
                                _ => grass.clone(),
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    x as f32 * TILE_SIZE,
                                    -(y as f32) * TILE_SIZE,
                                    0.,
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Name::new(match c {
                            'g' => "Grass",
                            'd' => "Dirt",
                            _ => "Background"
                        }))
                        .id(),
                );
            }
            if current_char_count > bg_char_count {
                bg_char_count = current_char_count;
            }
        }
    }

    let file = File::open("assets/maps/path.ob").expect("No map found");


    let mut ob_line_count = 0_usize;
    let mut ob_char_count = 0_usize;
    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            let mut current_char_count = 0_usize;
            ob_line_count += 1;
            for (x, c) in line.chars().enumerate() {
                current_char_count += 1;
                if c != 'f' {
                    continue;
                }
                tiles.push(
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(TILE_SIZE)),
                                ..default()
                            },
                            texture: match c {
                                'f' => fence.clone(),
                                _ => fence.clone(),
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    x as f32 * TILE_SIZE,
                                    -(y as f32) * TILE_SIZE,
                                    1.,
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Name::new(match c {
                            'f' => "Fence",
                            _ => "Foreground"
                        }))
                        .insert(TileCollider)
                        .id(),
                );
            }
            if current_char_count > ob_char_count {
                ob_char_count = current_char_count;
            }
        }
    }

    let line_count = usize::max(bg_line_count, ob_line_count);
    let char_count = usize::max(bg_char_count, ob_char_count);

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform {
            translation: Vec3::new(-(char_count as f32 * TILE_SIZE) / 2., (line_count as f32 * TILE_SIZE) / 2., 0.),
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .push_children(&tiles);
}
