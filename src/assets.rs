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

    #[asset(path = "bubble.png")]
    pub bubble: Handle<Image>,
}
