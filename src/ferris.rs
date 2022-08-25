use crate::assets::MyAssets;
use crate::camera::CameraTarget;
use crate::spritesheet::{Spritesheet, SpritesheetAnimation};
use crate::{Despawn, GameState};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, EntityInstance};
use bevy_rapier2d::prelude::*;

// tunables
const LINEAR_DAMPING: f32 = 0.7;
const BALL_LINEAR_DAMPING: f32 = 1.0;
const FRICTION: f32 = 0.5;
const BALL_FRICTION: f32 = 0.3;
const RESTITUTION: f32 = 0.3;
const BALL_RESTITUTION: f32 = 2.0;

const WALK_IMPULSE_GROUND: f32 = 0.3;
const WALK_IMPULSE_AIR: f32 = 0.05;
const JUMP_IMPULSE: f32 = 4.0;

const JUMP_IMPULSE_BALL: f32 = 4.0;

const _ROT_IMPULSE: f32 = 0.00005;

const JUMP_TIMEOUT: f32 = 0.3;
const LETHAL_VELOCITY: f32 = -150.0;

const WALKING: bool = true;

const MAX_WALK_VEL: f32 = 90.0;

const FERRIS_Z: f32 = 1.0;
const BUBBLE_Z: f32 = 2.0;

#[derive(Clone, Bundle)]
pub struct FerrisBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub external_impulse: ExternalImpulse,
    pub player_input_targtet: PlayerInputTarget,
    pub camera_target: CameraTarget,
    pub ccd: Ccd,
    pub velocity: Velocity,
    pub locked_axes: LockedAxes,
    pub restitution: Restitution,
    pub damping: Damping,
    pub friction: Friction,
    pub collider_mass_properties: ColliderMassProperties,
    pub ground_state: GroundState,
    pub gravity_scale: GravityScale,
}

impl Default for FerrisBundle {
    fn default() -> Self {
        FerrisBundle::bubble()
    }
}

impl FerrisBundle {
    pub fn walking() -> Self {
        FerrisBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::convex_hull(&[
                Vec2::new(5.0, -5.0),
                Vec2::new(-5.0, -5.0),
                Vec2::new(-7.0, 3.0),
                Vec2::new(0.0, 8.0),
                Vec2::new(7.0, 3.0),
            ])
            .unwrap(),
            external_impulse: ExternalImpulse::default(),
            player_input_targtet: PlayerInputTarget,
            camera_target: CameraTarget,
            ccd: Ccd { enabled: true },
            velocity: Velocity::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            restitution: Restitution {
                coefficient: RESTITUTION,
                ..default()
            },
            damping: Damping {
                linear_damping: LINEAR_DAMPING,
                ..default()
            },
            friction: Friction {
                coefficient: FRICTION,
                ..default()
            },
            collider_mass_properties: default(),
            ground_state: default(),
            gravity_scale: default(),
        }
    }

    pub fn bubble() -> Self {
        FerrisBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(14.0),
            external_impulse: ExternalImpulse::default(),
            player_input_targtet: PlayerInputTarget,
            camera_target: CameraTarget,
            ccd: Ccd { enabled: true },
            velocity: Velocity::default(),
            locked_axes: default(),
            restitution: Restitution {
                coefficient: BALL_RESTITUTION,
                ..default()
            },
            damping: Damping {
                linear_damping: BALL_LINEAR_DAMPING,
                angular_damping: 1.0,
            },
            friction: Friction {
                coefficient: BALL_FRICTION,
                ..default()
            },
            collider_mass_properties: ColliderMassProperties::Density(0.3),
            ground_state: GroundState::default().with_bubble(),
            gravity_scale: GravityScale(0.5),
        }
    }
}

impl From<EntityInstance> for FerrisBundle {
    fn from(_: EntityInstance) -> Self {
        FerrisBundle::default()
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct FerrisLdtkBundle {
    #[from_entity_instance]
    #[bundle]
    pub spawnpoint_bundle: FerrisBundle,
}

#[derive(Component, Default, Clone)]
pub struct FerrisSpawnpoint;

#[derive(Component, Clone)]
pub struct PlayerInputTarget;

#[derive(Component, Clone)]
pub struct GroundState {
    on_ground: bool,
    jump_timer: Timer,
    dead: bool,
    terminal_velocity: bool,

    // FIXME: this stuff does not belong in ground-state
    wobble: bool,
    in_bubble: bool,
}

#[derive(Component)]
pub struct Bubble {
    wobble_timer: Timer,
}

impl Default for GroundState {
    fn default() -> Self {
        Self {
            on_ground: false,
            jump_timer: Timer::from_seconds(JUMP_TIMEOUT, false),
            dead: false,
            terminal_velocity: false,
            wobble: false,
            in_bubble: false,
        }
    }
}

impl GroundState {
    fn with_bubble(mut self) -> Self {
        self.in_bubble = true;
        self
    }
}

pub enum SpawnFerrisType {
    Walking,
    Bubble,
}

pub struct SpawnFerrisEvent {
    pub t: SpawnFerrisType,
    pub pos: Vec3,
    pub despawn: bool,
}

#[allow(clippy::type_complexity)]
fn spawn_ferris_system(
    mut commands: Commands,
    my_assets: Option<Res<MyAssets>>,
    spritesheets: Res<Assets<Spritesheet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ldtk_added_query: Query<Entity, Added<EntityInstance>>,
) {
    let my_assets = if let Some(my_assets) = my_assets {
        my_assets
    } else {
        return;
    };

    for entity in &ldtk_added_query {
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
        entity_commands
            .insert(TextureAtlasSprite {
                index: 0,
                ..default()
            })
            .insert(texture_atlas.clone());

        entity_commands
            .insert(animation)
            .insert_bundle(FerrisBundle::bubble())
            .insert(Name::new("ferris"));

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
            .insert(ImpulseJoint::new(entity, joint));
    }
}

fn player_input_system(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut ExternalImpulse,
            &mut GroundState,
            &Velocity,
            &Transform,
        ),
        With<PlayerInputTarget>,
    >,
    mut event_writer: EventWriter<SpawnFerrisEvent>,
) {
    for (entity, mut external_impulse, mut ground_state, velocity, transform) in &mut query {
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

        if input.pressed(KeyCode::A) {
            impulse_h -= walk_impulse;
        }
        if input.pressed(KeyCode::D) {
            impulse_h += walk_impulse;
        }
        // } else {
        //     if input.pressed(KeyCode::A) {
        //         impulse_h -= WALK_IMPULSE_AIR;
        //         rot_impulse += ROT_IMPULSE;
        //     }
        //     if input.pressed(KeyCode::D) {
        //         impulse_h += WALK_IMPULSE_AIR;

        //         rot_impulse -= ROT_IMPULSE;
        //     }
        // }
        if ground_state.in_bubble && input.just_pressed(KeyCode::P) {
            // event_writer.send(SpawnFerrisEvent {
            //     t: SpawnFerrisType::Walking,
            //     pos: transform.translation,
            //     despawn: true,
            // })
            commands
                .entity(entity)
                .remove_bundle::<FerrisBundle>()
                .insert_bundle(FerrisBundle::walking());
        } else if !ground_state.in_bubble && input.just_pressed(KeyCode::P) {
            // event_writer.send(SpawnFerrisEvent {
            //     t: SpawnFerrisType::Bubble,
            //     pos: transform.translation,
            //     despawn: true,
            // })
            commands
                .entity(entity)
                .remove_bundle::<FerrisBundle>()
                .insert_bundle(FerrisBundle::bubble());
        }

        if (ground_state.on_ground || ground_state.in_bubble)
            && ground_state.jump_timer.finished()
            && input.pressed(KeyCode::Space)
        {
            impulse_v += jump_impulse;
            ground_state.jump_timer.reset();
        }

        if impulse_h.signum() == velocity.linvel.x.signum()
            && velocity.linvel.x.abs() > MAX_WALK_VEL
        {
            info!("clamp walk");
            impulse_h = 0.0;
        }

        external_impulse.impulse.x = impulse_h;
        external_impulse.impulse.y = impulse_v;
        // external_impulse.torque_impulse = rot_impulse;
    }
}

fn ground_trace_system(
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
        if ground_state.wobble {
            info!("wobble");
        }
    }
}

fn _adjust_friction_system(mut query: Query<(&mut Friction, &GroundState)>) {
    for (mut friction, ground_state) in &mut query {
        friction.coefficient = if ground_state.on_ground { 3.0 } else { 0.0 };
    }
}

#[allow(clippy::collapsible_else_if)]
fn adjust_animation_system(
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
            let rolls = ["roll1", "roll2", "roll3", "roll0"];

            let pi_2 = std::f32::consts::PI / 2.0;

            let (mut angle, _, _) = transform.rotation.to_euler(EulerRot::ZXY);
            if angle < 0.0 {
                angle += std::f32::consts::PI * 2.0;
            }
            // let angle = transform.rotation.angle_between(Quat::from_rotation_z(0.0));
            // info!("angle: {}", angle);

            // let (axis, angle) = transform.rotation.to_axis_angle();
            // info!("angle: {:?} {}", axis, angle);
            let quat = ((angle / pi_2) as usize).clamp(0, rolls.len());
            // let name = if (0.0..pi4).contains(&angle) {
            //     "roll0"
            // } else if (pi4..(2.0* pi4)).contains(&angle)

            if animation.active_animation != rolls[quat] {
                animation.start_animation(rolls[quat], true);
            }
        }
    }
}

fn death_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &GroundState,
            &mut SpritesheetAnimation,
            &mut LockedAxes,
            &mut Velocity,
        ),
        With<PlayerInputTarget>,
    >,
) {
    for (entity, ground_state, mut animation, mut locked_axes, mut velocity) in &mut query {
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

            // 'hard impact': lock translation and zero velocity to prevent further physics (bounce back)
            *locked_axes = LockedAxes::all();
            velocity.linvel = Vec2::ZERO;
        }
    }
}

#[allow(clippy::type_complexity)]
fn bubble_wobble_system(
    time: Res<Time>,
    mut bubble_query: Query<(&mut Bubble, &mut Transform)>,
    ferris_query: Query<&GroundState, (With<PlayerInputTarget>, Without<Bubble>)>,
) {
    if let Ok((mut bubble, mut bubble_transform)) = bubble_query.get_single_mut() {
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
        let l = 1.0
            - (bubble.wobble_timer.elapsed_secs() / bubble.wobble_timer.duration().as_secs_f32());

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
            SystemSet::on_update(GameState::InGame)
                .label(system_labels::Ground)
                .with_system(ground_trace_system),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(system_labels::Input)
                .after(system_labels::Ground)
                .with_system(player_input_system), // .with_system(adjust_friction_system),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(system_labels::Other)
                .after(system_labels::Input)
                .with_system(death_system.after(adjust_animation_system)),
        );
        // app.add_system_set(SystemSet::on_update(GameState::InGame));

        app.add_system(bubble_wobble_system)
            .add_system(spawn_ferris_system)
            .add_system(adjust_animation_system)
            // .add_system(spawn_ferris_at_spawnpoint)
            ;
        // app.add_system_to_stage(CoreStage::First, spawn_ferris_system);

        app.add_event::<SpawnFerrisEvent>();
        // app.register_ldtk_entity::<FerrisLdtkBundle>("Player");
    }
}
