use crate::spritesheet::Spritesheet;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
#[derive(AssetCollection)]
pub struct MyAssets {
    #[asset(path = "world.ldtk")]
    pub world: Handle<LdtkAsset>,

    #[asset(path = "ferris2.0.json")]
    pub ferris_spritesheet: Handle<Spritesheet>,

    #[asset(path = "ferris2.0.png")]
    pub ferris_atlas: Handle<Image>,

    #[asset(path = "bubble.json")]
    pub bubble_spritesheet: Handle<Spritesheet>,

    #[asset(path = "bubble.png")]
    pub bubble: Handle<Image>,

    #[asset(path = "bubble_single.png")]
    pub bubble_single: Handle<Image>,

    #[asset(path = "firework_orange.png")]
    pub firework_orange: Handle<Image>,

    #[asset(path = "firework.png")]
    pub firework: Handle<Image>,
}
