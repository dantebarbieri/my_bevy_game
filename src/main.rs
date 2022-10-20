use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>
    ),
        With<Player>
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let next_sprite = keyboard_input.any_pressed([KeyCode::Left, KeyCode::Right]);
            sprite.index = if next_sprite {
                (sprite.index + next_sprite as usize) % texture_atlas.textures.len()
            } else {
                0
            };
            if !keyboard_input.pressed(KeyCode::Right) && keyboard_input.pressed(KeyCode::Left) {
                sprite.flip_x = true;
            } else if keyboard_input.pressed(KeyCode::Right) && !keyboard_input.pressed(KeyCode::Left) {
                sprite.flip_x = false;
            }
        }
    }
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}