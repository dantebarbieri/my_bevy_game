use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{CameraTimer, TILE_SIZE, CameraProperties, tilemap::TileCollider};

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
            .add_system(player_movement.label("player_movement"))
            .add_system(player_animation)
            .add_system(camera_follow.after("player_movement"));
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
    wall_query: Query<&GlobalTransform, (With<TileCollider>, Without<Player>)>
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
        } * TILE_SIZE
            * time.delta_seconds();
    }
    
    if !wall_collision_check(transform.translation + Vec3::new(movement.x, 0., 0.), &wall_query) {
        movement.x = 0.;
    }
    if !wall_collision_check(transform.translation + Vec3::new(0., movement.y, 0.), &wall_query) {
        movement.y = 0.;
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
    player.running = player.motion != MoveStatus::Stopped && keyboard_input.pressed(KeyCode::LShift);
    if player.running {
        timer.set_duration(Duration::from_millis(
            (TIMER_DURATION * player.speed / player.run_speed * 1000.) as u64,
        ));
    } else {
        timer.set_duration(Duration::from_millis((TIMER_DURATION * 1000.) as u64));
    }
    transform.translation += movement;
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&GlobalTransform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.8),
            wall_transform.translation(),
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

fn camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut CameraTimer, &CameraProperties), (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    for (mut camera_transform, mut timer, properties) in &mut camera_query {
        timer.tick(time.delta());
        if properties.follow_distance == 0. {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        } else {
        if timer.just_finished() {
            let vel = Vec3::new(
                player_transform.translation.x - camera_transform.translation.x,
                player_transform.translation.y - camera_transform.translation.y,
                0.,
            );
            camera_transform.translation += vel / properties.follow_distance;
        }
        }
    }
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
