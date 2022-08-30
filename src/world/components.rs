use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Component, Default, Clone, Debug)]
pub struct Wall;

#[derive(Component, Copy, Clone)]
pub enum Item {
    ExitDoor,
    Key,
    Bubble,
    Spike,
    Unknown,
}

impl Default for Item {
    fn default() -> Self {
        Item::Unknown
    }
}

#[derive(Bundle, Clone, Default)]
pub struct ItemBundle {
    // transform: Transform,
    item: Item,
}

impl From<EntityInstance> for ItemBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        let item = if entity_instance.identifier == "Exit" {
            Item::ExitDoor
        } else if entity_instance.identifier == "Key" {
            Item::Key
        } else if entity_instance.identifier == "Bubble" {
            Item::Bubble
        } else if entity_instance.identifier == "Spike" {
            Item::Spike
        } else {
            Item::Unknown
        };

        ItemBundle {
            // transform: Transform::from_xyz(
            //     entity_instance.px.x as f32,
            //     entity_instance.px.y as f32,
            //     2.0,
            // ),
            item,
        }
    }
}

// #[derive(Clone, Default, Bundle, LdtkEntity)]
// pub struct PlayerBundle {
//     #[from_entity_instance]
//     #[bundle]
//     pub spawnpoint_bundle: SpawnpointBundle,
// }

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ItemBundleLdtk {
    #[from_entity_instance]
    #[bundle]
    pub exit_bundle: ItemBundle,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}
