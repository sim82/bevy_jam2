use bevy::prelude::*;

pub mod spritesheet;
pub mod ferris {}

pub struct MyPlugins;

impl PluginGroup for MyPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(spritesheet::SpritesheetPlugin);
    }
}
