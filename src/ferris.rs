use crate::assets::MyAssets;
use crate::camera::CameraTarget;
use crate::spritesheet::{Spritesheet, SpritesheetAnimation};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Clone)]
pub struct FerrisSpawnpoint;

#[derive(Component)]
pub struct PlayerInputTarget;

#[derive(Component)]
pub struct GroundState {
    on_ground: bool,
    jump_timer: Timer,
    dead: bool,
    terminal_velocity: bool,
}

impl Default for GroundState {
    fn default() -> Self {
        Self {
            on_ground: false,
            jump_timer: Timer::from_seconds(0.3, false),
            dead: false,
            terminal_velocity: false,
        }
    }
}

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
        animation.start_animation("walk left", true);

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                texture_atlas: texture_atlas.clone(),
                transform: *transform,
                ..default()
            })
            .insert(animation)
            .insert(RigidBody::Dynamic)
            // .insert(Collider::cuboid(6.0, 6.0))
            // .insert(Collider::round_cuboid(7.0, 5.0, 1.0))
            .insert(
                Collider::convex_hull(&[
                    Vec2::new(5.0, -5.0),
                    Vec2::new(-5.0, -5.0),
                    Vec2::new(-7.0, 3.0),
                    Vec2::new(0.0, 8.0),
                    Vec2::new(7.0, 3.0),
                ])
                .unwrap(),
            )
            // .insert(Damping {
            //     linear_damping: 1.0,
            //     ..default()
            // })
            .insert(Friction {
                coefficient: 3.0,
                ..default() // combine_rule: todo!(),
            })
            // .insert(Restitution {
            //     coefficient: 0.2,
            //     ..default()
            // })
            .insert(ExternalImpulse::default())
            .insert(ExternalForce::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(PlayerInputTarget)
            .insert(CameraTarget)
            .insert(Ccd { enabled: false })
            .insert(GroundState::default())
            .insert(Velocity::default());
    }
}

fn player_input_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (&mut ExternalImpulse, &mut ExternalForce, &mut GroundState),
        With<PlayerInputTarget>,
    >,
) {
    for (mut external_impulse, mut external_force, mut ground_state) in &mut query {
        ground_state.jump_timer.tick(time.delta());

        let walk_impulse = if ground_state.on_ground { 0.4 } else { 0.05 };
        let jump_impulse = 4.0;

        let mut force_h = 0.0;
        let mut force_v = 0.0;
        if input.pressed(KeyCode::A) {
            force_h -= walk_impulse;
        }
        if input.pressed(KeyCode::D) {
            force_h += walk_impulse;
        }
        if ground_state.on_ground
            && ground_state.jump_timer.finished()
            && input.pressed(KeyCode::Space)
        {
            force_v += jump_impulse;
            ground_state.jump_timer.reset();
        }

        external_impulse.impulse.x = force_h;
        external_impulse.impulse.y = force_v;

        // external_force.force.x = force_h;
        // external_force.force.y = force_v;

        // info!("impulse: {:?} {:?}", external_impulse, external_force);
    }
}

fn ground_trace_system(
    rapier_context: Res<RapierContext>,
    mut query: Query<(&mut GroundState, &Transform, &Collider, &Velocity)>,
) {
    for (mut ground_state, transform, collider, velocity) in &mut query {
        let collider = Collider::cuboid(5.0, 6.0);
        let ground_res = rapier_context.cast_shape(
            transform.translation.xy(),
            Rot::default(),
            Vec2::Y * -1.0,
            &collider,
            1.0,
            QueryFilter::only_fixed(),
        );
        ground_state.on_ground = ground_res.is_some();
        ground_state.terminal_velocity = velocity.linvel.y < -180.0;
        if ground_state.on_ground && ground_state.terminal_velocity {
            info!("deadly impact: {}", velocity.linvel.y);
            ground_state.dead = true;
        }
    }
}

fn adjust_friction_system(mut query: Query<(&mut Friction, &GroundState)>) {
    for (mut friction, ground_state) in &mut query {
        friction.coefficient = if ground_state.on_ground { 3.0 } else { 0.0 };
    }
}

#[allow(clippy::collapsible_else_if)]
fn adjust_animation_system(
    mut query: Query<(&GroundState, &Velocity, &mut SpritesheetAnimation), With<PlayerInputTarget>>,
) {
    for (ground_state, velocity, mut animation) in &mut query {
        let walking = velocity.linvel.x.abs() > 0.2;
        let vel_right = velocity.linvel.x >= 0.0;

        if ground_state.on_ground {
            if walking {
                if vel_right && animation.active_animation != "walk right" {
                    animation.start_animation("walk right", true);
                } else if !vel_right && animation.active_animation != "walk left" {
                    animation.start_animation("walk left", true);
                }
            } else if !walking && animation.active_animation != "stand" {
                animation.start_animation("stand", true)
            }
        } else {
            if ground_state.terminal_velocity && animation.active_animation != "panic" {
                animation.start_animation("panic", true);
            } else if !ground_state.terminal_velocity
                && vel_right
                && animation.active_animation != "jump right"
            {
                animation.start_animation("jump right", true);
            } else if !ground_state.terminal_velocity
                && !vel_right
                && animation.active_animation != "jump left"
            {
                animation.start_animation("jump left", true);
            }
        }
    }
}

fn death_system(
    mut commands: Commands,
    mut query: Query<(Entity, &GroundState, &mut SpritesheetAnimation), With<PlayerInputTarget>>,
) {
    for (entity, ground_state, mut animation) in &mut query {
        // info!("on ground: {:?}", ground_state.on_ground);

        // if ground_state.on_ground {
        //     info!("on ground");

        // }
        if ground_state.terminal_velocity {
            info!("terminal velocity");
        }
        if ground_state.on_ground && ground_state.terminal_velocity {
            animation.start_animation("die", false);
            commands
                .entity(entity)
                .remove::<PlayerInputTarget>()
                .remove::<GroundState>()
                .insert(crate::DespawnToCorpse);
        }
    }
}

pub struct FerrisPlugin;

mod system_labels {
    use bevy::prelude::*;
    #[derive(SystemLabel)]
    pub struct Ground;

    #[derive(SystemLabel)]
    pub struct Input;

    #[derive(SystemLabel)]
    pub struct Other;
}

impl Plugin for FerrisPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::new()
                .label(system_labels::Ground)
                .with_system(ground_trace_system),
        );

        app.add_system_set(
            SystemSet::new()
                .label(system_labels::Input)
                .after(system_labels::Ground)
                .with_system(player_input_system)
                .with_system(adjust_friction_system),
        );

        app.add_system_set(
            SystemSet::new()
                .label(system_labels::Other)
                .after(system_labels::Input)
                .with_system(spawn_ferris_system)
                .with_system(adjust_animation_system)
                .with_system(death_system.after(adjust_animation_system)),
        );
    }
}
