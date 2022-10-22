use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::TILE_SIZE;

// use crate::sprites::Characters;

#[derive(Clone, Copy, Default, Eq, Inspectable, PartialEq)]
enum Direction {
    Down,
    Left,
    Up,
    #[default]
    Right,
}

#[derive(Clone, Copy, Eq, Inspectable, PartialEq)]
enum MoveStatus {
    Stopped,
    Moving(Direction),
}

#[derive(Component, Inspectable)]
pub struct Player {
    motion: MoveStatus,
    running: bool,
    speed: f32,
    run_speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(player_animation);
    }
}

pub struct PlayerAtlas(Handle<TextureAtlas>);

const TIMER_DURATION: f32 = 0.1;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let gabe_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let gabe_atlas = TextureAtlas::from_grid(gabe_handle, Vec2::new(24.0, 24.0), 7, 1);
    let gabe_atlas_handle = texture_atlases.add(gabe_atlas);

    commands.insert_resource(PlayerAtlas(gabe_atlas_handle.clone()));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            texture_atlas: gabe_atlas_handle,
            transform: Transform {
                translation: Vec3::Z,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            motion: MoveStatus::Stopped,
            running: false,
            speed: 4.,
            run_speed: 6.,
        })
        .insert(AnimationTimer(Timer::from_seconds(TIMER_DURATION, true)));
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut AnimationTimer, &mut Transform)>,
) {
    let (mut player, mut timer, mut transform) = query.single_mut();
    let mut movement = Vec3::ZERO;
    for code in keyboard_input.get_pressed() {
        movement += match code {
            KeyCode::W => Vec3::Y,
            KeyCode::S => Vec3::NEG_Y,
            KeyCode::A => Vec3::NEG_X,
            KeyCode::D => Vec3::X,
            _ => Vec3::ZERO,
        } * if player.running {
            player.run_speed
        } else {
            player.speed
        } * TILE_SIZE * time.delta_seconds();
    }
    player.motion = if movement.x > 0. {
        MoveStatus::Moving(Direction::Right)
    } else if movement.x < 0. {
        MoveStatus::Moving(Direction::Left)
    } else if movement.y > 0. {
        MoveStatus::Moving(Direction::Up)
    } else if movement.y < 0. {
        MoveStatus::Moving(Direction::Down)
    } else {
        MoveStatus::Stopped
    };
    player.running = keyboard_input.pressed(KeyCode::LShift);
    if player.running {
        timer.set_duration(Duration::from_millis((TIMER_DURATION * player.speed / player.run_speed * 1000.) as u64));
    } else {
        timer.set_duration(Duration::from_millis((TIMER_DURATION * 1000.) as u64));
    }
    transform.translation += movement;
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn player_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    player_sheet: Res<PlayerAtlas>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (player, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(&player_sheet.0).unwrap();
            sprite.index = if player.motion != MoveStatus::Stopped {
                sprite.index + 1
            } else {
                0
            } % texture_atlas.textures.len();
            sprite.flip_x = match player.motion {
                MoveStatus::Stopped => sprite.flip_x,
                MoveStatus::Moving(direction) => match direction {
                    Direction::Left => true,
                    Direction::Right => false,
                    _ => sprite.flip_x,
                },
            }
        }
    }
}
