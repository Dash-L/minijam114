use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Fonts {
    #[asset(path = "fonts/iosevka.ttf")]
    pub main: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 3, rows = 1))]
    #[asset(path = "sprites/player-sheet.png")]
    pub player: Handle<TextureAtlas>,
}
