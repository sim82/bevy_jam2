use bevy::prelude::*;
#[derive(Debug)]
pub struct FerrisConfigureEvent {
    pub entity: Entity,
    pub bubble: bool,
}
