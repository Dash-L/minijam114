use std::f32::consts::PI;

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
    pub bullet_type: Vec<Handle<Image>>,
    #[asset(paths("sprites/spread1.png", "sprites/spread2.png"), collection(typed))]
    pub spread: Vec<Handle<Image>>,
    #[asset(paths("sprites/ice.png", "sprites/suc.png"), collection(typed))]
    pub effects: Vec<Handle<Image>>,
    #[asset(paths("sprites/lock.png", "sprites/unlock.png"), collection(typed))]
    pub locks: Vec<Handle<Image>>,
    #[asset(path = "sprites/rocket.png")]
    pub rocket: Handle<Image>,
    #[asset(path = "sprites/sawblade.png")]
    pub saw_blade: Handle<Image>,
    #[asset(path = "sprites/bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "sprites/player-base.png")]
    pub base: Handle<Image>,
    #[asset(path = "sprites/zombie.png")]
    pub enemy: Handle<Image>,
}

pub struct Spread(pub u32, pub f32);

impl Spread {
    pub fn next(&mut self) {
        if self.0 == 0 {
            self.1 = PI / 16.0;

            self.0 += 1;
        } else if self.0 == 1 {
            self.1 = 0.0;

            self.0 += 1;
        }
    }
}

impl Default for Spread {
    fn default() -> Spread {
        Spread(0, PI / 8.0)
    }
}

#[derive(Default)]
pub enum BulletType {
    #[default]
    Regular = 0,
    Rocket = 1,
    SawBlade = 2,
}

#[derive(Default)]
pub struct HasIce(pub bool);

#[derive(Default)]
pub struct HasSuck(pub bool);

#[derive(Default)]
pub struct MousePosition(pub Vec2);

#[derive(Default)]
pub struct Coins(pub u32);
