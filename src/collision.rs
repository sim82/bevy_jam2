use std::ops::Range;

use crate::world::Wall;
use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::IntegrationParameters};

#[derive(Component)]
struct ColliderRoot;

fn spawn_wall_collider_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Wall>>,
    root_query: Query<Entity, With<ColliderRoot>>,
) {
    if query.is_empty() {
        return;
    }

    let tile_size = Vec2::new(16.0, 16.0);
    let tile_halfsize = tile_size / 2.0;

    // cluster tiles by row
    let mut by_row = HashMap::<u32, Vec<u32>>::new();
    for (_entity, transform) in &query {
        let tile_int = (transform.translation.xy() - tile_halfsize) / 16.0;
        let x = tile_int.x as u32;
        let y = tile_int.y as u32;

        match by_row.entry(y) {
            bevy::utils::hashbrown::hash_map::Entry::Occupied(mut e) => {
                e.get_mut().push(x);
            }
            bevy::utils::hashbrown::hash_map::Entry::Vacant(e) => {
                e.insert(vec![x]);
            }
        }
    }

    debug!("by rows: {:?}", by_row);

    // find contiguous runs in rows (i.e. merge horizontally)
    let mut by_run = HashMap::<Range<u32>, Vec<u32>>::new();
    for (y, mut row) in by_row {
        row.sort();
        for run in row.group_by(|a, b| *a + 1 == *b) {
            debug!("run: {} {:?}", y, run);
            let first = run[0];
            let last = *run.last().unwrap();
            match by_run.entry(first..last + 1) {
                bevy::utils::hashbrown::hash_map::Entry::Occupied(mut e) => e.get_mut().push(y),
                bevy::utils::hashbrown::hash_map::Entry::Vacant(e) => {
                    e.insert(vec![y]);
                }
            }
        }
    }

    let mut collider_entities = Vec::new();
    // find contiguous 'stacks' of row runs (i.e. merge vertically)
    for (h_run, mut y) in by_run {
        y.sort();
        debug!("h: {:?}: {:?}", h_run, y);
        for v_run in y.group_by(|a, b| (*a + 1) == *b) {
            let p0 = Vec2::new(h_run.start as f32 * 16.0, v_run[0] as f32 * 16.0);
            let p1 = Vec2::new(
                h_run.end as f32 * 16.0,
                (v_run.last().unwrap() + 1) as f32 * 16.0,
            );

            let mid = (p0 + p1) / 2.0;
            let halfsize = (p1 - p0) / 2.0;
            info!("{:?} {:?}", mid, halfsize);
            let entity = commands
                .spawn()
                .insert_bundle(SpatialBundle {
                    transform: Transform::from_translation(mid.extend(0.0)),
                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(halfsize.x, halfsize.y))
                .id();
            collider_entities.push(entity);
        }
    }

    let root = root_query.get_single().unwrap_or_else(|_| {
        commands
            .spawn_bundle(SpatialBundle::default())
            .insert(Name::new("colliders"))
            .id()
    });

    commands
        .entity(root)
        .insert_children(0, &collider_entities[..]);
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_wall_collider_system)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(64.0)) // assume that ferris is about 25cm tall
            .insert_resource(RapierConfiguration {
                gravity: Vec2::Y * -9.81 * 20.0,
                ..default()
            })
        // .add_plugin(RapierDebugRenderPlugin {
        //     mode: DebugRenderMode::all(),
        //     ..default()
        // })
        ;
    }
}
