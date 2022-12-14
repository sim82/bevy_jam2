use super::components::*;
use super::constants::*;
use super::events::*;
use crate::{
    assets::MyAssets,
    spritesheet::{Spritesheet, SpritesheetAnimation},
    world::PlayerSpawnState,
    Despawn,
};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_ldtk::{prelude::*, EntityInstance};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::time::Duration;

/// Spawn player character on level start.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn spawn_ferris_system(
    mut commands: Commands,
    my_assets: Option<Res<MyAssets>>,
    spritesheets: Res<Assets<Spritesheet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ldtk_added_query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    mut event_writer: EventWriter<FerrisConfigureEvent>,
    mut player_spawn_state: ResMut<PlayerSpawnState>,
    level_selection: Res<LevelSelection>,
) {
    let my_assets = if let Some(my_assets) = my_assets {
        my_assets
    } else {
        return;
    };

    for (entity, entity_instance) in &ldtk_added_query {
        if entity_instance.identifier != "Player" {
            continue;
        }
        info!("spawn ferris: {:?}", entity);

        player_spawn_state.spawned = true;

        let spritesheet = spritesheets.get(&my_assets.ferris_spritesheet).unwrap();
        info!("spritesheet: {:?}", spritesheet);
        let num_frames = spritesheet.durations.len();
        let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
            my_assets.ferris_atlas.clone(),
            Vec2::splat(16.0),
            num_frames,
            1,
        ));

        let mut animation = SpritesheetAnimation::new(my_assets.ferris_spritesheet.clone());
        animation.start_animation("walk left", true);

        let mut entity_commands = commands.entity(entity);
        let entity = entity_commands
            .insert(TextureAtlasSprite {
                index: 0,
                ..default()
            })
            .insert(texture_atlas.clone())
            .insert(animation)
            .insert(Name::new("ferris"))
            .insert_bundle(FerrisPersistentBundle::default())
            .id();

        match level_selection.as_ref() {
            LevelSelection::Identifier(name) if name == "End" => {
                entity_commands.insert(CelebrationMode {
                    dir_timer: Timer::from_seconds(0.5, true),
                    jump_timer: Timer::from_seconds(0.5, true),
                    right: true,
                });
            }
            _ => {}
        }

        event_writer.send(FerrisConfigureEvent {
            entity,
            bubble: false,
        });
    }
}

/// Apply user input. Player control is completely based on rapier physics, using external-impulse.
#[allow(clippy::type_complexity)]
pub fn player_input_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut ExternalImpulse,
            &mut GroundState,
            &Velocity,
            &mut Transform,
        ),
        (With<PlayerInputTarget>, Without<CelebrationMode>),
    >,
    mut event_writer: EventWriter<FerrisConfigureEvent>,
) {
    for (entity, mut external_impulse, mut ground_state, velocity, mut transform) in &mut query {
        ground_state.jump_timer.tick(time.delta());

        // info!("velocity: {:?}", velocity);

        let mut impulse_h = 0.0;
        let mut impulse_v = 0.0;

        // let mut rot_impulse = 0.0;

        let jump_impulse = if WALKING {
            JUMP_IMPULSE
        } else {
            JUMP_IMPULSE_BALL
        };

        // if WALKING {
        let walk_impulse = if ground_state.on_ground {
            WALK_IMPULSE_GROUND
        } else {
            WALK_IMPULSE_AIR
        };

        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            impulse_h -= walk_impulse;
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            impulse_h += walk_impulse;
        }
        if cfg!(feature = "inspector") {
            if ground_state.in_bubble && input.just_pressed(KeyCode::P) {
                event_writer.send(FerrisConfigureEvent {
                    entity,
                    bubble: false,
                })
                // change_to_walking(&mut commands, entity);
            } else if !ground_state.in_bubble && input.just_pressed(KeyCode::P) {
                event_writer.send(FerrisConfigureEvent {
                    entity,
                    bubble: true,
                })

                // change_to_bubble(&mut commands, entity);
            }
        }

        if (ground_state.on_ground || ground_state.in_bubble)
            && ground_state.jump_timer.finished()
            && (input.pressed(KeyCode::Space)
                || input.pressed(KeyCode::Up)
                || input.pressed(KeyCode::W))
        {
            impulse_v += jump_impulse;
            ground_state.jump_timer.reset();
        }

        if impulse_h.signum() == velocity.linvel.x.signum()
            && velocity.linvel.x.abs() > MAX_WALK_VEL
        {
            // info!("clamp walk");
            impulse_h = 0.0;
        }

        external_impulse.impulse.x = impulse_h;
        external_impulse.impulse.y = impulse_v;
        // external_impulse.torque_impulse = rot_impulse;
        transform.translation.z = FERRIS_Z; // crappy hack
    }
}

/// Special mode for end screen. Disable user-imput and jump around randomly.
pub fn player_celebrate_system(
    time: Res<Time>,
    mut query: Query<(&mut ExternalImpulse, &mut CelebrationMode, &GroundState)>,
) {
    let mut rng = rand::thread_rng();
    for (mut external_impulse, mut celebraton_mode, ground_state) in &mut query {
        celebraton_mode.dir_timer.tick(time.delta());
        celebraton_mode.jump_timer.tick(time.delta());

        if celebraton_mode.dir_timer.just_finished() {
            celebraton_mode.right = !celebraton_mode.right;
            celebraton_mode
                .dir_timer
                .set_duration(Duration::from_secs_f32(rng.gen_range(0.2..0.5)));
        }
        let impulse_h = if celebraton_mode.right {
            WALK_IMPULSE_GROUND
        } else {
            -WALK_IMPULSE_GROUND
        };

        let impulse_v = if celebraton_mode.jump_timer.just_finished() {
            celebraton_mode
                .jump_timer
                .set_duration(Duration::from_secs_f32(rng.gen_range(0.5..1.0)));
            JUMP_IMPULSE
        } else {
            0.0
        };

        if ground_state.on_ground {
            external_impulse.impulse.x = impulse_h;
            external_impulse.impulse.y = impulse_v;
        }
    }
}

/// re-configure character into walk or bubble mode. Removes and re-adds the transient
/// character components, but leaves persistent stuff (e.g. transformation, items...) in place
pub fn reconfigure_ferris_system(
    mut commands: Commands,
    mut event_reader: EventReader<FerrisConfigureEvent>,
    my_assets: Option<Res<MyAssets>>,
    spritesheets: Res<Assets<Spritesheet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<&mut Transform, Without<Bubble>>,
    bubble_query: Query<(Entity, &ImpulseJoint, &Transform), With<Bubble>>,
) {
    let my_assets = if let Some(my_assets) = my_assets {
        my_assets
    } else {
        return;
    };
    for event in event_reader.iter() {
        info!("reconfigure ferris: {:?}", event);
        for (bubble_entity, joint, bubble_transform) in &bubble_query {
            if joint.parent == event.entity {
                commands.entity(bubble_entity).despawn();
                info!("despawn bubble {:?}", bubble_entity);

                let mut animation = SpritesheetAnimation::new(my_assets.bubble_spritesheet.clone());
                animation.start_animation("bubble", true);

                let spritesheet = spritesheets.get(&my_assets.bubble_spritesheet).unwrap();

                let num_frames = spritesheet.durations.len();
                let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
                    my_assets.bubble.clone(),
                    Vec2::splat(28.0),
                    num_frames,
                    1,
                ));

                let mut animation = SpritesheetAnimation::new(my_assets.bubble_spritesheet.clone());
                animation.start_animation("pop", true);

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: 0,
                            ..default()
                        },
                        texture_atlas: texture_atlas.clone(),
                        transform: Transform::from_translation(
                            bubble_transform.translation.xy().extend(BUBBLE_Z),
                        ),
                        ..default()
                    })
                    .insert(animation)
                    .insert(Despawn::TimeToLive(0.5));
            }
        }

        // reset rotation
        if let Ok(mut transform) = query.get_mut(event.entity) {
            transform.rotation = default();
        }
        if event.bubble {
            commands
                .entity(event.entity)
                .remove_bundle::<FerrisBundle>()
                .insert_bundle(FerrisBundle::bubble());

            let joint = RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, 0.0))
                .local_anchor2(Vec2::new(0.0, 0.0));

            commands
                .spawn_bundle(SpriteBundle {
                    texture: my_assets.bubble_single.clone(),
                    transform: Transform::from_translation(Vec3::Z * BUBBLE_Z),
                    ..default()
                })
                .insert(Bubble {
                    wobble_timer: Timer::from_seconds(3.0, false),
                })
                .insert(RigidBody::Dynamic)
                .insert(AdditionalMassProperties::Mass(0.0001))
                .insert(ImpulseJoint::new(event.entity, joint))
                .insert(Despawn::OnLevelEnd);
        } else {
            commands
                .entity(event.entity)
                .remove_bundle::<FerrisBundle>()
                .insert_bundle(FerrisBundle::walking());
        }
    }
}

/// Check ground state of character by downwards shape-cast of the collision shape.
/// In walk mode it uses smaller cube shape to make it less sensitive on sidewards impatcts.
/// Also checks impact velocity for lethality and bubble wobble systems.
pub fn ground_trace_system(
    rapier_context: Res<RapierContext>,
    mut query: Query<(&mut GroundState, &Transform, &Collider, &Velocity)>,
) {
    for (mut ground_state, transform, collider, velocity) in &mut query {
        let collider = if !ground_state.in_bubble {
            Collider::cuboid(5.0, 6.0)
        } else {
            collider.clone()
        };
        let ground_res = rapier_context.cast_shape(
            transform.translation.xy(),
            Rot::default(),
            Vec2::Y * -1.0,
            &collider,
            1.0,
            QueryFilter::only_fixed(),
        );
        ground_state.on_ground = ground_res.is_some();
        ground_state.terminal_velocity = velocity.linvel.y < LETHAL_VELOCITY;
        if ground_state.on_ground && ground_state.terminal_velocity {
            info!("deadly impact: {}", velocity.linvel.y);
            ground_state.dead = true;
        }

        ground_state.wobble = ground_state.on_ground && velocity.linvel.y < -20.0;
        // if ground_state.wobble {
        //     info!("wobble");
        // }
    }
}

fn _adjust_friction_system(mut query: Query<(&mut Friction, &GroundState)>) {
    for (mut friction, ground_state) in &mut query {
        friction.coefficient = if ground_state.on_ground { 3.0 } else { 0.0 };
    }
}

/// Control the character animation according to the current state (velocity, ground contact etc)
#[allow(clippy::collapsible_else_if)]
pub fn adjust_animation_system(
    mut query: Query<
        (
            &GroundState,
            &Velocity,
            &mut SpritesheetAnimation,
            &Transform,
        ),
        With<PlayerInputTarget>,
    >,
) {
    for (ground_state, velocity, mut animation, transform) in &mut query {
        if !ground_state.in_bubble {
            // walk mode: mainly react to ground state and velocity

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
        } else {
            // bubble mode: rotate eyes
            let rolls = ["roll1", "roll2", "roll3", "roll0"];

            let pi_2 = std::f32::consts::PI / 2.0;

            let (mut angle, _, _) = transform.rotation.to_euler(EulerRot::ZXY);
            if angle < 0.0 {
                angle += std::f32::consts::PI * 2.0;
            }
            let quat = ((angle / pi_2) as usize).clamp(0, rolls.len() - 1);

            if animation.active_animation != rolls[quat] {
                animation.start_animation(rolls[quat], true);
            }
        }
    }
}

/// Check for lethal impacts etc and run death animation (+despawn)
#[allow(clippy::type_complexity)]
pub fn death_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &GroundState,
            &mut SpritesheetAnimation,
            &mut LockedAxes,
            &mut Velocity,
            &Transform,
        ),
        With<PlayerInputTarget>,
    >,
) {
    for (entity, ground_state, mut animation, mut locked_axes, mut velocity, transform) in
        &mut query
    {
        if ground_state.terminal_velocity {
            debug!("terminal velocity");
        }
        if (ground_state.on_ground && ground_state.terminal_velocity)
            || transform.translation.y < -20.0
        {
            animation.start_animation("die", false);
            commands
                .entity(entity)
                .remove::<PlayerInputTarget>()
                .remove::<GroundState>()
                .insert(crate::DespawnToCorpse);

            // 'hard impact': lock translation and zero velocity to prevent further physics (bounce back)
            *locked_axes = LockedAxes::all();
            velocity.linvel = Vec2::ZERO;
        }
    }
}

/// Control bubble wobble animation depending on ground impact
/// The basic idea is to have three frequency components per dimension, where the
/// higher frequency components are blended in after impacts.
#[allow(clippy::type_complexity)]
pub fn bubble_wobble_system(
    time: Res<Time>,
    mut bubble_query: Query<(&mut Bubble, &mut Transform)>,
    ferris_query: Query<&GroundState, (With<PlayerInputTarget>, Without<Bubble>)>,
) {
    for (mut bubble, mut bubble_transform) in &mut bubble_query {
        bubble.wobble_timer.tick(time.delta());
        if let Ok(ground_state) = ferris_query.get_single() {
            if ground_state.wobble {
                bubble.wobble_timer.reset();
            }
        }

        // combine (pun intended) three wobble modes per dimension. try to get low periodicity
        let tx1 = time.seconds_since_startup() * 1.5;
        let ty1 = time.seconds_since_startup() * 2.5;
        let tx2 = time.seconds_since_startup() * 8.5;
        let ty2 = time.seconds_since_startup() * 7.0;
        let tx3 = time.seconds_since_startup() * 19.0;
        let ty3 = time.seconds_since_startup() * 23.0;

        // blend between low and high frequency wobble modes, based on wobble-timer
        let l = bubble.wobble_timer.percent_left();

        let c1 = 0.05 * (1.0 - l);
        let c2 = 0.05 * l;
        let c3 = 0.025 * l;

        // info!("l: {}", l);

        bubble_transform.scale.x =
            1.0 + (tx1.sin() as f32) * c1 + (tx2.cos() as f32) * c2 + (tx3.sin() as f32) * c3;
        bubble_transform.scale.y =
            1.0 + (ty1.sin() as f32) * c1 + (ty2.cos() as f32) * c2 + (ty3.sin() as f32) * c3;
    }
}
