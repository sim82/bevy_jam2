use crate::world::Wall;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::IntegrationParameters};

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
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(64.0))
            .insert_resource(RapierConfiguration {
                gravity: Vec2::Y * -9.81 * 20.0,
                ..default()
            })
            // .insert_resource(RapierPhysicsPlugin::pixels_per_meter(50.0))
            // .insert_resource(IntegrationParameters{
            //     allowed_linear_error: 0.1,
            // //    dt: 1.0/240.0,
            //     // max_ccd_substeps: 5,
            //     ..default()
            // })
            // .add_plugin(RapierDebugRenderPlugin::default())
            ;
    }
}