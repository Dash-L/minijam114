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
    #[asset(path = "sprites/player-base.png")]
    pub base: Handle<Image>,
    #[asset(path = "sprites/zombie.png")]
    pub enemy: Handle<Image>,
}

#[derive(Default)]
pub struct MousePosition(pub Vec2);
