use crate::world::Wall;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

fn spawn_wall_collider_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Wall>>,
) {
    let tile_size = Vec2::new(16.0, 16.0);
    for (entity, transform) in &query {
        info!("tile: {:?}", transform);
        commands
            .entity(entity)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(tile_size.x / 2.0, tile_size.y / 2.0));
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_wall_collider_system)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default());
    }
}
