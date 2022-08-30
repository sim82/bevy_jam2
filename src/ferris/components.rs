use super::constants::*;
use crate::camera::CameraTarget;
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, EntityInstance};
use bevy_rapier2d::prelude::*;
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Keys {
    pub key1: bool,
}

#[derive(Clone, Bundle, Default)]
pub struct FerrisPersistentBundle {
    pub keys: Keys,
}

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

// #[derive(Component, Default, Clone)]
// pub struct FerrisSpawnpoint;

#[derive(Component, Clone)]
pub struct PlayerInputTarget;

#[derive(Component, Clone)]
pub struct GroundState {
    pub on_ground: bool,
    pub jump_timer: Timer,
    pub dead: bool,
    pub terminal_velocity: bool,

    // FIXME: this stuff does not belong in ground-state
    pub wobble: bool,
    pub in_bubble: bool,
}

#[derive(Component)]
pub struct Bubble {
    pub wobble_timer: Timer,
}

#[derive(Component)]
pub struct CelebrationMode {
    pub dir_timer: Timer,
    pub right: bool,
    pub jump_timer: Timer,
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
