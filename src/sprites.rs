use std::ops::Range;

use bevy::{prelude::*, asset::LoadState};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RpgSpriteHandles>()
        .init_resource::<Characters>()
        .init_resource::<Tiles>()
        .init_resource::<Props>()
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup));
    }
}

#[derive(Debug, Default)]
pub struct Characters{
    pub gabe: Handle<TextureAtlas>,
    pub hat_guy: Handle<Image>,
    pub mani: Handle<TextureAtlas>,
    pub sensei: Handle<Image>,
}

#[derive(Debug, Default)]
pub struct Tiles{
    pub slice: Handle<Image>,
    pub waterfall_atlas: Handle<TextureAtlas>,
    pub misc_atlas: Handle<TextureAtlas>
}

#[derive(Debug, Default)]
pub struct Props{
    pub barrel_atlas: Handle<TextureAtlas>,
    pub board_atlas: Handle<TextureAtlas>,
    pub bridge: Handle<Image>,
    pub crate_atlas: Handle<TextureAtlas>,
    pub fence_complete: Handle<Image>,
    pub fence_raw_atlas: Handle<TextureAtlas>,
    pub fence_atlas: Handle<TextureAtlas>,
    pub fish_atlas: Handle<TextureAtlas>,
    pub flower_atlas: Handle<TextureAtlas>,
    pub flowers: Handle<Image>,
    pub grass_atlas: Handle<TextureAtlas>,
    pub house_inn: Handle<Image>,
    pub loot_atlas: Handle<TextureAtlas>,
    pub mini_lake: Handle<Image>,
    pub rock_atlas: Handle<TextureAtlas>,
    pub rod: Handle<Image>,
    pub treasure_atlas: Handle<TextureAtlas>,
    pub tree_atlas: Handle<TextureAtlas>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.handles = asset_server.load_folder("textures/rpg").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>
) {
    commands.insert_resource(load_characters(&asset_server, &mut texture_atlases));
    commands.insert_resource(load_tiles(&asset_server, &mut texture_atlases, &mut textures));
    commands.insert_resource(load_props(&asset_server, &mut texture_atlases, &mut textures));
}

fn load_characters(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>
) -> Characters {
    let gabe_handle = asset_server.get_handle("textures/rpg/chars/gabe/gabe-idle-run.png");
    let gabe_atlas = TextureAtlas::from_grid(gabe_handle, Vec2::new(24.0, 24.0), 7, 1);
    let gabe_atlas_handle = texture_atlases.add(gabe_atlas);

    let hat_guy_handle = asset_server.get_handle("textures/rpg/chars/hat-guy/hat-guy.png");

    let mani_handle = asset_server.get_handle("textures/rpg/chars/mani/mani-idle-run.png");
    let mani_atlas = TextureAtlas::from_grid(mani_handle, Vec2::new(24.0, 24.0), 7, 1);
    let mani_atlas_handle = texture_atlases.add(mani_atlas);

    let sensei_handle = asset_server.get_handle("textures/rpg/chars/mani/mani-idle-run.png");

    Characters{
        gabe: gabe_atlas_handle,
        hat_guy: hat_guy_handle,
        mani: mani_atlas_handle,
        sensei: sensei_handle
    }
}

fn load_tiles(
    asset_server: &AssetServer,
    mut texture_atlases: &mut Assets<TextureAtlas>,
    mut textures: &mut Assets<Image>
) -> Tiles {
    let slice_handle = asset_server.get_handle("textures/rpg/tiles/generic-rpg-Slice.png");

    let waterfall_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..8, "textures/rpg/tiles/generic-rpg-tile-waterfall{idx}.png");

    let misc_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..72, "textures/rpg/tiles/generic-rpg-tile{idx}.png");

    Tiles{
        slice: slice_handle,
        waterfall_atlas: waterfall_atlas_handle,
        misc_atlas: misc_atlas_handle
    }
}

fn load_props(
    asset_server: &AssetServer,
    mut texture_atlases: &mut Assets<TextureAtlas>,
    mut textures: &mut Assets<Image>
) -> Props {
    let barrel_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..4, "textures/rpg/props/generic-rpg-barrel{idx}.png");
    
    let board_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..5, "textures/rpg/props/generic-rpg-board{idx}.png");

    let bridge_handle = asset_server.get_handle("textures/rpg/props/generic-rpg-bridge.png");

    let crate_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..4, "textures/rpg/props/generic-rpg-crate{idx}.png");

    let fence_complete_handle = asset_server.get_handle("textures/rpg/props/generic-rpg-fence-complete.png");

    let fence_atlas_raw_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..26, "textures/rpg/props/generic-rpg-fence-raw{idx}.png");

    let fence_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..15, "textures/rpg/props/generic-rpg-fence{idx}.png");

    let fish_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..5, "textures/rpg/props/generic-rpg-fish{idx}.png");

    let flower_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..4, "textures/rpg/props/generic-rpg-flower{idx}.png");

    let flowers_handle = asset_server.get_handle("textures/rpg/props/generic-rpg-flowers.png");

    let grass_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..3, "textures/rpg/props/generic-rpg-grass{idx}.png");

    let house_inn_handle = asset_server.get_handle("textures/rpg/props/generic-rpg-house-inn.png");

    let loot_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..6, "textures/rpg/props/generic-rpg-loot{idx}.png");

    let mini_lake_handle = asset_server.get_handle("textures/rpg/props/generic-rpg-mini-lake.png");

    let rock_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..7, "textures/rpg/props/generic-rpg-rock{idx}.png");

    let rod_handle = asset_server.get_handle("textures/rpg/props/generic-rpg-rod.png");

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let handle = asset_server.get_handle("textures/rpg/props/generic-rpg-trasure-open.png");
    let texture = textures.get(&handle).expect("Textures folder contained a file which way matched by a loader which did not create an `Image` asset");
    texture_atlas_builder.add_texture(handle, texture);
    let handle = asset_server.get_handle("textures/rpg/props/generic-rpg-treasure-closed.png");
    let texture = textures.get(&handle).expect("Textures folder contained a file which way matched by a loader which did not create an `Image` asset");
    texture_atlas_builder.add_texture(handle, texture);
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let treasure_atlas_handle = texture_atlases.add(texture_atlas);

    let tree_atlas_handle = load_atlas(&asset_server, &mut texture_atlases, &mut textures, 1..3, "textures/rpg/props/generic-rpg-tree{idx}.png");

    Props{
        barrel_atlas: barrel_atlas_handle,
        board_atlas: board_atlas_handle,
        bridge: bridge_handle,
        crate_atlas: crate_atlas_handle,
        fence_complete: fence_complete_handle,
        fence_raw_atlas: fence_atlas_raw_handle,
        fence_atlas: fence_atlas_handle,
        fish_atlas: fish_atlas_handle,
        flower_atlas: flower_atlas_handle,
        flowers: flowers_handle,
        grass_atlas: grass_atlas_handle,
        house_inn: house_inn_handle,
        loot_atlas: loot_atlas_handle,
        mini_lake: mini_lake_handle,
        rock_atlas: rock_atlas_handle,
        rod: rod_handle,
        treasure_atlas: treasure_atlas_handle,
        tree_atlas: tree_atlas_handle,
    }
}

pub fn load_atlas(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    mut textures: &mut Assets<Image>,
    range: Range<i32>,
    path: &str
) -> Handle<TextureAtlas> {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for idx in range {
        // FIXME: only supports 2 digits
        let idx = format!("{:02}", idx);
        let path = path.replace("{idx}", idx.as_str());
        println!("{path}");
        let handle = asset_server.get_handle(path);
        let texture = textures.get(&handle).expect("Textures folder contained a file which way matched by a loader which did not create an `Image` asset");
        texture_atlas_builder.add_texture(handle, texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    texture_atlases.add(texture_atlas)
}