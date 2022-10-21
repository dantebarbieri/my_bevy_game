use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Clone, Copy, Inspectable)]
enum Direction {
    Down,
    Left,
    Up,
    Right
}

#[derive(Component, Inspectable)]
pub struct Player {
    facing: Direction
}

pub struct PlayerSheet(pub Handle<TextureAtlas>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(player_animation);
    }
}

fn spawn_player(mut commands: Commands, gabe: Res<PlayerSheet>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: gabe.0.clone(),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            facing: Direction::Down
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    const PLAYER_SPEED: f32 = 100.;

    let (mut player, mut transform) = query.single_mut();
    for code in keyboard_input.get_just_pressed() {
        player.facing = match code {
            KeyCode::W => Direction::Up,
            KeyCode::S => Direction::Down,
            KeyCode::A => Direction::Left,
            KeyCode::D => Direction::Right,
            _ => player.facing
        }
    }
    for code in keyboard_input.get_pressed() {
        transform.translation += match code {
            KeyCode::W => Vec3::Y,
            KeyCode::S => Vec3::NEG_Y,
            KeyCode::A => Vec3::NEG_X,
            KeyCode::D => Vec3::X,
            _ => Vec3::ZERO
        } * PLAYER_SPEED * time.delta_seconds();
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn player_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    player_sheet: Res<PlayerSheet>,
    mut query: Query<(
        &Player,
        &mut AnimationTimer,
        &mut TextureAtlasSprite
    )>,
) {
    for (player, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(&player_sheet.0).unwrap();
            let next_sprite = keyboard_input.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]);
            sprite.index = if next_sprite {
                (sprite.index + next_sprite as usize) % texture_atlas.textures.len()
            } else {
                0
            };
            sprite.flip_x = match player.facing {
                Direction::Left | Direction::Up => true,
                Direction::Right | Direction::Down => false
            }
        }
    }
}