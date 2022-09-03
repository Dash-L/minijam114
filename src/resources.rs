use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Fonts {
    #[asset(path = "fonts/iosevka.ttf")]
    pub main: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 2, rows = 1))]
    #[asset(path = "sprites/barrel-sheet.png")]
    pub barrel: Handle<TextureAtlas>,
    #[asset(paths("sprites/rocket.png", "sprites/sawblade.png"), collection(typed))]
    pub bullettype: Vec<Handle<Image>>,
    #[asset(paths("sprites/spread1.png", "sprites/spread2.png"), collection(typed))]
    pub spread: Vec<Handle<Image>>,
    #[asset(paths("sprites/ice.png", "sprites/suc.png"), collection(typed))]
    pub effects: Vec<Handle<Image>>,
    #[asset(paths("sprites/lock.png", "sprites/unlock.png"), collection(typed))]
    pub locks: Vec<Handle<Image>>,
    #[asset(path = "sprites/player-base.png")]
    pub base: Handle<Image>,
    #[asset(path = "sprites/zombie.png")]
    pub enemy: Handle<Image>,
}

#[derive(Default)]
pub struct MousePosition(pub Vec2);
