use bevy::prelude::*;

/// Re-configure the player character
#[derive(Debug)]
pub struct FerrisConfigureEvent {
    /// Target entity to reconfigure.
    pub entity: Entity,

    /// Enable bubble mode
    pub bubble: bool,
}
