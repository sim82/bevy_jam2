use crate::assets::MyAssets;
use crate::spritesheet::{Spritesheet, SpritesheetAnimation};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Clone)]
pub struct FerrisSpawnpoint;

#[derive(Component)]
pub struct PlayerInputTarget;

fn spawn_ferris_system(
    mut commands: Commands,
    my_assets: Option<Res<MyAssets>>,
    spritesheets: Res<Assets<Spritesheet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    added_query: Query<(Entity, &Transform), Added<FerrisSpawnpoint>>,
) {
    if added_query.is_empty() {
        return;
    }
    let my_assets = if let Some(my_assets) = my_assets {
        my_assets
    } else {
        return;
    };

    let spritesheet = spritesheets.get(&my_assets.ferris_spritesheet).unwrap();
    info!("spritesheet: {:?}", spritesheet);
    let num_frames = spritesheet.durations.len();
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        my_assets.ferris_atlas.clone(),
        Vec2::splat(16.0),
        num_frames,
        1,
    ));

    for (entity, transform) in &added_query {
        let mut animation = SpritesheetAnimation::new(my_assets.ferris_spritesheet.clone());
        animation.start_animation("walk left");

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                texture_atlas: texture_atlas.clone(),
                transform: transform.clone(),
                ..default()
            })
            .insert(animation)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(6.0, 6.0))
            .insert(Friction {
                coefficient: 3.0,
                ..default() // combine_rule: todo!(),
            })
            .insert(ExternalImpulse::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(PlayerInputTarget)
            .insert(Ccd { enabled: true });
    }
}

fn player_input_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut ExternalImpulse, With<PlayerInputTarget>>,
) {
    for mut external_impulse in &mut query {
        let walk_impulse = 1000.0;
        let jump_impulse = 1000.0;

        let mut force_h = 0.0;
        let mut force_v = 0.0;
        if input.pressed(KeyCode::A) {
            force_h -= walk_impulse;
        }
        if input.pressed(KeyCode::D) {
            force_h += walk_impulse;
        }
        if input.pressed(KeyCode::Space) {
            force_v += jump_impulse;
        }

        external_impulse.impulse.x = force_h;
        external_impulse.impulse.y = force_v;
        info!("impulse: {:?}", external_impulse);
    }
}

pub struct FerrisPlugin;

impl Plugin for FerrisPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_ferris_system)
            .add_system(player_input_system);
    }
}
